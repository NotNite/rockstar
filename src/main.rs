#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

mod api;
mod repl;
mod util;

#[derive(Parser)]
struct Args {
    /// The path to your Lua script
    script: PathBuf,

    /// Enter a REPL after evaluating the script
    #[clap(short, long)]
    repl: bool,
}

fn setup_lua() -> anyhow::Result<mlua::Lua> {
    let lua = mlua::Lua::new();
    let rockstar = api::Rockstar::new();

    let rockstar_arc = Arc::new(Mutex::new(rockstar));

    lua.set_app_data(Arc::clone(&rockstar_arc));

    {
        let globals = lua.globals();
        globals.set("rockstar", Arc::clone(&rockstar_arc))?;
    }

    Ok(lua)
}

#[cfg(target_os = "windows")]
fn fix_stdout() {
    use windows::Win32::System::Console::{AttachConsole, ATTACH_PARENT_PROCESS};

    unsafe {
        AttachConsole(ATTACH_PARENT_PROCESS);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    fix_stdout();

    let args = Args::parse();
    let lua = setup_lua()?;

    let rockstar = {
        let _rockstar = lua.app_data_ref::<Arc<Mutex<api::Rockstar>>>().unwrap();
        Arc::clone(&_rockstar)
    };

    let script = std::fs::read_to_string(&args.script)?;
    lua.load(&script).exec_async().await?;

    let lua = Arc::new(Mutex::new(lua));

    let mut futures = Vec::new();

    let lua_event_loop = Arc::clone(&lua);
    futures.push(tokio::task::spawn(async move {
        api::events::run_event_loop(rockstar, lua_event_loop).await;
    }));

    if args.repl {
        let lua_repl = Arc::clone(&lua);
        futures.push(tokio::task::spawn(async move {
            repl::run_repl(lua_repl).await;
        }));
    }

    futures::future::join_all(futures).await;

    Ok(())
}

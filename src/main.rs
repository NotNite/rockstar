#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use clap::Parser;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

mod api;
mod util;

#[derive(Parser)]
struct Args {
    /// The path to your Lua script
    script: PathBuf,
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
    api::events::run_event_loop(rockstar, lua).await;

    Ok(())
}

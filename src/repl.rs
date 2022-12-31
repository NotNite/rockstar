use mlua::{Error, Lua, MultiValue};
use rustyline::Editor;
use std::sync::{Arc, Mutex};

// basically stolen from https://github.com/khvzak/mlua/blob/master/examples/repl.rs
pub async fn run_repl(lua: Arc<Mutex<Lua>>) {
    let mut editor = Editor::<()>::new().expect("Failed to make rustyline editor");

    loop {
        let mut prompt = "> ";
        let mut line = String::new();

        loop {
            match editor.readline(prompt) {
                Ok(input) => line.push_str(&input),
                Err(_) => return,
            }

            let lua = lua.lock().unwrap();
            let eval = lua.load(&line).eval::<MultiValue>();
            match eval {
                Ok(_) => {
                    editor.add_history_entry(line);
                    break;
                }
                Err(Error::SyntaxError {
                    incomplete_input: true,
                    ..
                }) => {
                    line.push('\n');
                    prompt = ">> ";
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    break;
                }
            }
        }
    }
}

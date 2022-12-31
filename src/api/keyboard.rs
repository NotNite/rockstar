use mlua::UserData;
use rdev::EventType;

#[derive(Debug, Clone)]
pub struct Keyboard;

impl UserData for Keyboard {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_function("press", |_, key: String| async move {
            let cheating = format!("\"{}\"", key);
            let cheating: rdev::Key = serde_json::from_str(&cheating).unwrap();

            rdev::simulate(&EventType::KeyPress(cheating))
                .map_err(|_| mlua::Error::external("failed to press key"))?;
            //tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            rdev::simulate(&EventType::KeyRelease(cheating))
                .map_err(|_| mlua::Error::external("failed to release key"))?;

            Ok(())
        });
    }
}

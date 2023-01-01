use mlua::UserData;

#[derive(Debug, Clone)]
pub struct System;

impl UserData for System {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_function_get("os", |_, _| Ok(std::env::consts::OS));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        // implemented purely because requiring a native library sucks
        // why does lua have to be so minimal :(
        methods.add_function("list_dir", |lua, path: String| {
            let path = std::path::Path::new(&path);
            let entries = std::fs::read_dir(path).map_err(mlua::Error::external)?;

            let mut files = Vec::new();
            for entry in entries {
                let entry = entry.map_err(mlua::Error::external)?;
                let path = entry.path();
                let name = path.file_name().unwrap().to_str().unwrap().to_string();

                let table = lua.create_table()?;
                table.set("name", name)?;
                table.set("is_dir", path.is_dir())?;
                files.push(table);
            }

            Ok(files)
        });

        methods.add_function("notify", |_, (title, description): (String, String)| {
            notify_rust::Notification::new()
                .summary(&title)
                .body(&description)
                .show()
                .map_err(mlua::Error::external)?;

            Ok(())
        });
    }
}

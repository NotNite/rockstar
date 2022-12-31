use mlua::UserData;
use rdev::{simulate, Button, EventType};

#[derive(Debug, Clone)]
pub struct Mouse {
    pub left_pressed: bool,
    pub middle_pressed: bool,
    pub right_pressed: bool,

    pub mouse_x: f64,
    pub mouse_y: f64,
}

impl Mouse {
    pub fn new() -> Self {
        Self {
            left_pressed: false,
            middle_pressed: false,
            right_pressed: false,

            mouse_x: 0.,
            mouse_y: 0.,
        }
    }
}

impl UserData for Mouse {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        let click_check = |orig, new, button| {
            if orig != new {
                let a = if orig {
                    EventType::ButtonPress(button)
                } else {
                    EventType::ButtonRelease(button)
                };

                simulate(&a).unwrap();
            }
        };

        fields.add_field_method_get("left_pressed", |_, mouse| Ok(mouse.left_pressed));
        fields.add_field_method_get("middle_pressed", |_, mouse| Ok(mouse.middle_pressed));
        fields.add_field_method_get("right_pressed", |_, mouse| Ok(mouse.right_pressed));

        fields.add_field_method_set("left_pressed", move |_, mouse, value| {
            click_check(mouse.left_pressed, value, Button::Left);
            mouse.left_pressed = value;

            Ok(())
        });

        fields.add_field_method_set("middle_pressed", move |_, mouse, value| {
            click_check(mouse.middle_pressed, value, Button::Middle);
            mouse.middle_pressed = value;

            Ok(())
        });

        fields.add_field_method_set("right_pressed", move |_, mouse, value| {
            click_check(mouse.right_pressed, value, Button::Right);
            mouse.right_pressed = value;

            Ok(())
        });

        fields.add_field_method_get("x", |_, mouse| Ok(mouse.mouse_x));
        fields.add_field_method_get("y", |_, mouse| Ok(mouse.mouse_y));

        fields.add_field_method_set("x", |_, mouse, value| {
            simulate(&EventType::MouseMove {
                x: value,
                y: mouse.mouse_y,
            })
            .map_err(mlua::Error::external)?;
            mouse.mouse_x = value;

            Ok(())
        });

        fields.add_field_method_set("y", |_, mouse, value| {
            simulate(&EventType::MouseMove {
                x: mouse.mouse_x,
                y: value,
            })
            .unwrap();
            mouse.mouse_y = value;

            Ok(())
        });
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_async_function("click", move |_, button: u8| async move {
            let button = match button {
                1 => Button::Left,
                2 => Button::Middle,
                3 => Button::Right,
                _ => return Ok(()),
            };

            simulate(&EventType::ButtonPress(button)).map_err(mlua::Error::external)?;
            //tokio::time::sleep(std::time::Duration::from_millis(10)).await;
            simulate(&EventType::ButtonRelease(button)).map_err(mlua::Error::external)?;

            Ok(())
        });
    }
}

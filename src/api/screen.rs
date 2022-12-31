use captrs::Bgr8;
use mlua::UserData;

#[derive(Debug, Clone)]
pub struct Screen;

#[derive(Debug, Clone)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl UserData for Rgb {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("r", |_, rgb| Ok(rgb.r));
        fields.add_field_method_get("g", |_, rgb| Ok(rgb.g));
        fields.add_field_method_get("b", |_, rgb| Ok(rgb.b));
    }
}

#[derive(Debug, Clone)]
pub struct CaptureData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Bgr8>,
}

impl UserData for CaptureData {
    fn add_fields<'lua, F: mlua::UserDataFields<'lua, Self>>(fields: &mut F) {
        fields.add_field_method_get("width", |_, data| Ok(data.width));
        fields.add_field_method_get("height", |_, data| Ok(data.height));
    }

    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("get", |_, data, (x, y): (usize, usize)| {
            let index = y * data.width as usize + x;
            let bgr = data.data[index];

            Ok(Rgb {
                r: bgr.r,
                g: bgr.g,
                b: bgr.b,
            })
        });
    }
}

impl UserData for Screen {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("capture", |_, index: Option<usize>| {
            let mut capturer = captrs::Capturer::new(index.unwrap_or(0))
                .map_err(|_| mlua::Error::external("could not create capturer"))?;
            let image = capturer
                .capture_frame()
                .map_err(|_| mlua::Error::external("could not capture frame".to_string()))?;

            let geometry = capturer.geometry();
            let capture_data = CaptureData {
                width: geometry.0,
                height: geometry.1,
                data: image,
            };

            Ok(capture_data)
        });
    }
}

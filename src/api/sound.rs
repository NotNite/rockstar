use mlua::UserData;
use rodio::{cpal::traits::HostTrait, Device, DeviceTrait, OutputStream, Source};

#[derive(Clone, Debug)]
pub struct Sound;

impl UserData for Sound {
    fn add_methods<'lua, M: mlua::UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("play", |_, path: String| {
            let path = std::path::Path::new(&path);
            let file = std::fs::File::open(path).map_err(mlua::Error::external)?;

            let decoder = rodio::Decoder::new(std::io::BufReader::new(file))
                .map_err(mlua::Error::external)?;

            std::thread::spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();

                let length = decoder
                    .total_duration()
                    .unwrap_or(std::time::Duration::from_secs(0));

                stream_handle.play_raw(decoder.convert_samples()).unwrap();

                std::thread::sleep(length);
            });

            Ok(())
        });
    }
}

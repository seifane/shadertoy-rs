#![feature(slice_take)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

#[macro_use]
extern crate clap;
#[macro_use]
extern crate gfx;
extern crate gfx_window_glutin;
extern crate glutin;
extern crate image;
#[macro_use]
extern crate log;
extern crate anyhow;
extern crate env_logger;
extern crate notify;
extern crate old_school_gfx_glutin_ext;
extern crate reqwest;
extern crate serde_json;
extern crate hound;
extern crate cpal;
extern crate rb;
extern crate spectrum_analyzer;
extern crate gfx_device_gl;

mod argvalues;
mod download;
mod error;
mod loader;
mod runner;
mod audio;
mod channel;

use argvalues::ArgValues;
use audio::player::AudioPlayer;

fn main() {
    env_logger::init().expect("Unable to initialize logger");

    // let mut player = AudioPlayer::new("/home/tiemajor/Music/save-this-world.wav".to_string());
    // player.play();

    let argvalues = ArgValues::from_cli();
    match argvalues {
        Ok(vals) => {
            runner::run(vals).unwrap();
        },
        Err(e) => {
            error!("{}", e);
        },
    }
}

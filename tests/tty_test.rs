extern crate palm;

mod common;

use palm::tty;

#[test]
fn ports() {
    for it in tty::ports().unwrap() {
        println!("find {}", it);
    }
}

#[test]
fn uuid() {
    println!("{it} {it:?}", it = tty::Uuid::null());
    println!("{it} {it:?}", it = tty::Uuid::zero());
    println!("{it} {it:?}", it = "abcd".parse::<tty::Uuid>().unwrap());
    for _ in 1..100 {
        println!("{it} {it:?}", it = tty::Uuid::default());
    }
}

const NAME: &str = "tty";

#[test]
fn pull() {
    env_logger::init();
    loquat::audio::listen(NAME, common::get_tmp_file("audio/002.mp3")).unwrap();
}

#[test]
fn play() {
    env_logger::init();
    _play("audio/001.mp3").unwrap();
}

#[test]
fn stop() {
    env_logger::init();
    _stop().unwrap();
}

fn _stop() -> nut::Result<()> {
    __audio_stop!(NAME)
}

fn _play(name: &str) -> nut::Result<()> {
    let file = common::get_tmp_file(name);
    println!("play file {}", file.display());
    __audio_play!(NAME, common::get_tmp_file(name), 10, 95)
}

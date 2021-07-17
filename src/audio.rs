use std::ops::Deref;
use std::path::{Path, PathBuf};

use sdl2::mixer::{
    self, allocate_channels, get_chunk_decoder, get_chunk_decoders_number, get_music_decoder,
    get_music_decoders_number, open_audio, query_spec, InitFlag, Music, AUDIO_S16LSB,
    DEFAULT_CHANNELS,
};
use serde::{Deserialize, Serialize};

use super::{queue::zero::Queue, Result};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum Message {
    Play {
        file: PathBuf,
        count: i32,
        volume: i32,
    },
    Stop,
}

fn hook_finished() {
    info!("play ends!");
}

pub fn listen<P: AsRef<Path>>(name: &str, file: P) -> Result<()> {
    info!("start audio consumer thread {}", name);
    let queue = Queue::Ipc(name.to_string()).pull()?;

    debug!("sdl2 mixer linked version: {}", mixer::get_linked_version());
    let sdl = sdl2::init()?;
    let _audio = sdl.audio()?;
    open_audio(
        44_100,
        // signed 16 bit samples, in little-endian byte order
        AUDIO_S16LSB,
        // Stereo
        DEFAULT_CHANNELS,
        1_024,
    )?;
    let _mixer_context = sdl2::mixer::init(InitFlag::all())?;

    // Number of mixing channels available for sound effect `Chunk`s to play simultaneously.
    allocate_channels(4);

    {
        let n = get_chunk_decoders_number();
        debug!("available chunk(sample) decoders: {}", n);
        for i in 0..n {
            debug!("decoder {} => {}", i, get_chunk_decoder(i));
        }
    }

    {
        let n = get_music_decoders_number();
        debug!("available music decoders: {}", n);
        for i in 0..n {
            debug!("decoder {} => {}", i, get_music_decoder(i));
        }
    }

    debug!("query spec => {:?}", query_spec());
    Music::hook_finished(hook_finished);

    let mut music = Music::from_file(file.as_ref())?;
    debug!("init music => {:?}", music);
    loop {
        let msg = queue.recv_bytes(0)?;
        let msg = msg.deref();
        let msg: Message = flexbuffers::from_buffer(&msg)?;
        info!("receive message {:?}", msg);
        match msg {
            Message::Play {
                file,
                count,
                volume,
            } => {
                music = Music::from_file(file)?;
                Music::set_volume(volume);
                debug!("music => {:?}", music);
                debug!("music type => {:?}", music.get_type());
                debug!("music volume => {:?}", Music::get_volume());
                debug!("play => {:?}", music.play(count));
            }
            Message::Stop => {
                Music::halt();
            }
        };
    }
}

#[macro_export]
macro_rules! __try_grpc {
    ($r:expr, $s:expr) => {{
        $r.map_err(|x| x.to_string()).map_err($s)
    }};
    ($r:expr) => {{
        $r.map_err(|x| x.to_string())
            .map_err(tonic::Status::internal)
    }};
}

#[macro_export]
macro_rules! __tty_write {
    ($n:expr, $m:expr) => {{
        let q = nut::queue::zero::Queue::Ipc($n.to_string()).push()?;
        q.send($m, 0)?;
        Ok(())
    }};
}

#[macro_export]
macro_rules! __audio_play {
    ($n:expr, $f:expr, $c:expr, $v:expr) => {{
        let q = nut::queue::zero::Queue::Ipc($n.to_string()).push()?;
        let b = flexbuffers::to_vec(&loquat::audio::Message::Play {
            file: $f,
            count: $c,
            volume: $v,
        })?;
        q.send(&b, 0)?;
        Ok(())
    }};
}

#[macro_export]
macro_rules! __audio_stop {
    ($n:expr) => {{
        let q = nut::queue::zero::Queue::Ipc($n.to_string()).push()?;
        let b = flexbuffers::to_vec(&loquat::audio::Message::Stop)?;
        q.send(&b, 0)?;
        Ok(())
    }};
}

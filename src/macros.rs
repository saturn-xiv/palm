#[macro_export]
macro_rules! try_grpc {
    ($r:expr, $s:expr) => {{
        $r.map_err(|x| x.to_string()).map_err($s)
    }};
    ($r:expr) => {{
        $r.map_err(|x| x.to_string())
            .map_err(tonic::Status::internal)
    }};
}

#[macro_export]
macro_rules! current_user {
    ($s:expr, $r:expr) => {{
        let ss = Session::new($r);
        let db = try_grpc!($s.db.get())?;
        let db = db.deref();
        let jwt = $s.jwt.deref();
        try_grpc!(ss.current_user(db, jwt), Status::unauthenticated)?
    }};
}

#[macro_export]
macro_rules! to_timestamp {
    ($x:expr) => {{
        let it: std::time::SystemTime =
            chrono::DateTime::<chrono::Utc>::from_utc($x, chrono::Utc).into();
        it.into()
    }};
}

#[macro_export]
macro_rules! current_pi_user {
    ($s:expr, $r:expr) => {{
        let ss = Session::new($r);
        let db = try_grpc!($s.db.get())?;
        let db = db.deref();
        let aes = $s.aes.deref();
        let jwt = $s.jwt.deref();
        try_grpc!(ss.current_user(db, jwt, aes), Status::unauthenticated)?
    }};
}

// #[macro_export]
// macro_rules! __tty_write {
//     ($n:expr, $m:expr) => {{
//         let q = nut::queue::zero::Queue::Ipc($n.to_string()).push()?;
//         q.send($m, 0)?;
//         Ok(())
//     }};
// }

// #[macro_export]
// macro_rules! __audio_play {
//     ($n:expr, $f:expr, $c:expr, $v:expr) => {{
//         let q = nut::queue::zero::Queue::Ipc($n.to_string()).push()?;
//         let b = flexbuffers::to_vec(&loquat::audio::Message::Play {
//             file: $f,
//             count: $c,
//             volume: $v,
//         })?;
//         q.send(&b, 0)?;
//         Ok(())
//     }};
// }

// #[macro_export]
// macro_rules! __audio_stop {
//     ($n:expr) => {{
//         let q = nut::queue::zero::Queue::Ipc($n.to_string()).push()?;
//         let b = flexbuffers::to_vec(&loquat::audio::Message::Stop)?;
//         q.send(&b, 0)?;
//         Ok(())
//     }};
// }

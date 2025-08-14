use std::time::SystemTime;

pub(crate) fn get_timestamp(time: SystemTime) -> u128 {
    time.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

use std::sync::Mutex;

pub trait Identifiable {
    fn id(&self) -> i64;
}

static CID: Mutex<i64> = Mutex::new(0);

pub fn uuid() -> i64 {
    let mut id = CID.lock().unwrap();
    *id += 1;
    *id - 1
}

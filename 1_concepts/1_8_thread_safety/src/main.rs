use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};
use std::thread;

#[derive(Debug)]
struct OnlySync<'a>(MutexGuard<'a, String>);

#[derive(Debug)]
struct OnlySend(RefCell<String>);

#[derive(Debug)]
struct SyncAndSend(i32);

#[derive(Debug)]
struct NotSyncNotSend(Rc<i32>);

fn main() {
    let mutex = Mutex::new(String::from("hello"));

    let only_sync = OnlySync(mutex.lock().unwrap());
    let only_send = OnlySend(RefCell::new(String::from("hello")));
    let sync_and_send = SyncAndSend(8);
    let _not_sync_not_send = NotSyncNotSend(Rc::new(8));

    thread::scope(|s| {
        s.spawn(|| {
            dbg!(&only_sync); // works, because OnlySend is Sync
                              // dbg!(only_sync); // won't compile, because OnlySend isn't Send

            // dbg!(&only_send); // won't compile, because OnlySend isn't Sync
            dbg!(only_send); // works, because OnlySend is Send

            dbg!(&sync_and_send); // works, because SyncAndSend is Sync
            dbg!(sync_and_send); // works, because SyncAndSend is Send

            // dbg!(&_not_sync_not_send); // won't compile, because NotSyncNotSend isn't Sync
            // dbg!(_not_sync_not_send); // won't compile, because NotSyncNotSend isn't Send
        });
    });
}

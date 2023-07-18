use static_assertions::{
    assert_impl_all, assert_impl_one, assert_not_impl_all, assert_not_impl_any,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::MutexGuard;

struct OnlySync<'a>(MutexGuard<'a, String>);
struct OnlySend(RefCell<String>);
struct SyncAndSend(i32);
struct NotSyncNotSend(Rc<i32>);

fn main() {
    assert_impl_one!(OnlySync: Sync);
    assert_not_impl_any!(OnlySync: Send);

    assert_not_impl_any!(OnlySend: Sync);
    assert_impl_one!(OnlySend: Send);

    assert_impl_all!(SyncAndSend: Sync, Send);

    assert_not_impl_all!(NotSyncNotSend: Sync, Send);
}

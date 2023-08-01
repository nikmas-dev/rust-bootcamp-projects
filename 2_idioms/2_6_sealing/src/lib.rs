pub mod my_error;
pub mod my_iterator_ext;

pub use self::{my_error::MyError, my_iterator_ext::MyIteratorExt};
use std::any::TypeId;
use std::fmt::{Debug, Display, Formatter};

// struct TestSealing;
//
// impl Iterator for TestSealing {
//     type Item = ();
//
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }
// }
//
// impl crate::my_iterator_ext::private::Sealed for TestSealing {} // this won't compile
//
// impl MyIteratorExt for TestSealing {}

// struct TestSealing;
//
// impl Debug for TestSealing {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl Display for TestSealing {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }
//
// impl MyError for TestSealing {
//     fn type_id(&self, _: crate::my_error::private::Token) -> TypeId where Self: 'static { // this won't compile
//         todo!()
//     }
// }

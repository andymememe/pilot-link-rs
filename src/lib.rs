extern crate serde;

#[macro_use]
extern crate serde_derive;

use std::any::type_name;

pub mod protocol;
pub mod hotsync;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
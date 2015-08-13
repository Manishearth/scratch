#![feature(plugin, custom_derive)]
#![plugin(clippy)]
#![plugin(serde_macros)]
#![deny(let_unit_value)]
extern crate serde;

#[derive(Serialize)]
pub struct LayerId(pub usize, pub u32);

#[test]
fn it_works() {
}

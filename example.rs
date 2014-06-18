#![feature(default_type_params)]
#![allow(dead_code)]

use foundation::{NSObject, NSString, INSString};

mod runtime;
mod id;
mod foundation;

fn main() {
	let obj = NSObject::new();
	let obj2 = obj.clone();

	println!("{} == {}? {}", obj, obj2, obj == obj2);

	let obj3 = NSObject::new();
	println!("{} == {}? {}", obj, obj3, obj == obj3);

	let string = NSString::from_str("Hello, world!");
	println!("{}", string.as_str());
}
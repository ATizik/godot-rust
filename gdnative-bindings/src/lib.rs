#![allow(non_snake_case)] // because of the generated bindings.
#![allow(unused_imports)]

pub use gdnative_core::*;

use crate::get_api;
use crate::sys;

use libc;
use std::ops::*;
use std::sync::Once;

include!("bindings_types.rs");
include!("bindings_traits.rs");
include!("bindings_methods.rs");

#![cfg_attr(feature = "dox", feature(doc_cfg))]
#![deny(warnings)]
#![allow(clippy::single_component_path_imports)]

macro_rules! assert_initialized_main_thread {
    () => {
        if !::gtk::is_initialized_main_thread() {
            if ::gtk::is_initialized() {
                panic!("GTK may only be used from the main thread.");
            } else {
                panic!("GTK has not been initialized. Call `gtk::init` first.");
            }
        }
    };
}

/// No-op.
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(unused_imports)]
use gtk; // Required for the documentation to build without warnings

mod auto;

pub use auto::functions::*;
pub use auto::*;

mod manual;
pub use manual::*;

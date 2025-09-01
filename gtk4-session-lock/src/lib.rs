#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(warnings)]

use gtk4_session_lock_sys as ffi;

#[allow(unused_imports)]
#[allow(clippy::single_component_path_imports)]
use gdk; // Required for the documentation to build without warnings

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

mod auto;
pub use auto::functions::is_supported;

#[cfg(feature = "v1_1")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_1")))]
pub use auto::Instance;

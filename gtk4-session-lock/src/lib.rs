#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(warnings)]

use gtk4_session_lock_sys as ffi;

#[cfg(feature = "v1_1")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_1")))]
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

#[cfg(feature = "v1_1")]
#[cfg_attr(docsrs, doc(cfg(feature = "v1_1")))]
pub use auto::Instance;

mod manual;
pub use manual::is_supported;

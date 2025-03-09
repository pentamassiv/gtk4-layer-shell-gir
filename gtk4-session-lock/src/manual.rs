use glib::translate::from_glib;

use crate::ffi;

#[doc(alias = "gtk_session_lock_is_supported")]
pub fn is_supported() -> bool {
    unsafe { from_glib(ffi::gtk_session_lock_is_supported()) }
}

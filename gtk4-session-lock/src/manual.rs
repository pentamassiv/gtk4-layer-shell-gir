use glib::translate::from_glib;

use crate::ffi;

/// May block for a Wayland roundtrip the first time it's called.
///
/// # Returns
///
/// [`true`] if the platform is Wayland and Wayland compositor supports the
/// Session Lock protocol.
#[doc(alias = "gtk_session_lock_is_supported")]
pub fn is_supported() -> bool {
    unsafe { from_glib(ffi::gtk_session_lock_is_supported()) }
}

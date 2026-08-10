use std::ptr::NonNull;

use glib::object::IsA;
use glib::translate::*;
use gtk4_layer_shell_sys as ffi;

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// The underlying layer surface Wayland object
///
/// # Safety
///
/// The pointer is owned by gtk4-layer-shell and must not be freed. It remains
/// valid only while `window` is initialized as a layer surface; using it after
/// the surface is destroyed is undefined behavior.
#[doc(alias = "gtk_layer_get_zwlr_layer_surface_v1")]
#[doc(alias = "get_zwlr_layer_surface_v1")]
pub unsafe fn zwlr_layer_surface_v1(
    window: &impl IsA<gtk::Window>,
) -> Option<NonNull<ffi::zwlr_layer_surface_v1>> {
    assert_initialized_main_thread!();

    unsafe {
        NonNull::new(ffi::gtk_layer_get_zwlr_layer_surface_v1(
            window.as_ref().to_glib_none().0,
        ))
    }
}

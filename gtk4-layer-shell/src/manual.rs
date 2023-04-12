pub type ZwlrLayerSurfaceV1 = ffi::zwlr_layer_surface_v1;

/// ## `window`
/// A layer surface.
///
/// # Returns
///
/// The underlying layer surface Wayland object
#[cfg(any(feature = "v0_4", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v0_4")))]
#[doc(alias = "gtk_layer_get_zwlr_layer_surface_v1")]
#[doc(alias = "get_zwlr_layer_surface_v1")]
pub fn zwlr_layer_surface_v1(
    window: &impl glib::object::IsA<gtk::Window>,
) -> *mut ZwlrLayerSurfaceV1 {
    use glib::translate::ToGlibPtr;

    assert_initialized_main_thread!();
    unsafe { ffi::gtk_layer_get_zwlr_layer_surface_v1(window.as_ref().to_glib_none().0) }
}

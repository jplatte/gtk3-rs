// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct ActionBar(Object<ffi::GtkActionBar>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_action_bar_get_type(),
    }
}

impl ActionBar {
    #[cfg(feature = "v3_12")]
    pub fn new() -> ActionBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_action_bar_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_center_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_action_bar_get_center_widget(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn pack_end<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_end(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn pack_start<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_action_bar_pack_start(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_center_widget<T: IsA<Widget>>(&self, center_widget: Option<&T>) {
        unsafe {
            ffi::gtk_action_bar_set_center_widget(self.to_glib_none().0, center_widget.to_glib_none().0);
        }
    }
}

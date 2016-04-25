// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Container;
use Popover;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct PopoverMenu(Object<ffi::GtkPopoverMenu>): Popover, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_popover_menu_get_type(),
    }
}

impl PopoverMenu {
    #[cfg(feature = "v3_16")]
    pub fn new() -> PopoverMenu {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_popover_menu_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_16")]
    pub fn open_submenu(&self, name: &str) {
        unsafe {
            ffi::gtk_popover_menu_open_submenu(self.to_glib_none().0, name.to_glib_none().0);
        }
    }
}

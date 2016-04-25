// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use ColorChooser;
use Container;
use Widget;
use ffi;
use ffi::GtkColorButton;
use glib::object::Downcast;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gpointer;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct ColorButton(Object<ffi::GtkColorButton>): Button, Bin, Container, Widget, Actionable, ColorChooser;

    match fn {
        get_type => || ffi::gtk_color_button_get_type(),
    }
}

impl ColorButton {
    pub fn new() -> ColorButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_color_button_new()).downcast_unchecked()
        }
    }

    //pub fn new_with_color(color: /*Ignored*/&gdk::Color) -> ColorButton {
    //    unsafe { TODO: call ffi::gtk_color_button_new_with_color() }
    //}

    //pub fn new_with_rgba(rgba: /*Ignored*/&gdk::RGBA) -> ColorButton {
    //    unsafe { TODO: call ffi::gtk_color_button_new_with_rgba() }
    //}

    pub fn get_alpha(&self) -> u16 {
        unsafe {
            ffi::gtk_color_button_get_alpha(self.to_glib_none().0)
        }
    }

    //pub fn get_color(&self, color: /*Ignored*/gdk::Color) {
    //    unsafe { TODO: call ffi::gtk_color_button_get_color() }
    //}

    //pub fn get_rgba(&self, rgba: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_color_button_get_rgba() }
    //}

    pub fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_color_button_get_title(self.to_glib_none().0))
        }
    }

    pub fn get_use_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_color_button_get_use_alpha(self.to_glib_none().0))
        }
    }

    pub fn set_alpha(&self, alpha: u16) {
        unsafe {
            ffi::gtk_color_button_set_alpha(self.to_glib_none().0, alpha);
        }
    }

    //pub fn set_color(&self, color: /*Ignored*/&gdk::Color) {
    //    unsafe { TODO: call ffi::gtk_color_button_set_color() }
    //}

    //pub fn set_rgba(&self, rgba: /*Ignored*/&gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_color_button_set_rgba() }
    //}

    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_color_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    pub fn set_use_alpha(&self, use_alpha: bool) {
        unsafe {
            ffi::gtk_color_button_set_use_alpha(self.to_glib_none().0, use_alpha.to_glib());
        }
    }

    pub fn connect_color_set<F: Fn(&ColorButton) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ColorButton) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "color-set",
                transmute(color_set_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn color_set_trampoline(this: *mut GtkColorButton, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ColorButton) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

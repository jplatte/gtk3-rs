// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Align, Buildable, Container, ReliefStyle, ResizeMode, ToolItem, ToolShell, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkToolItemGroup")]
    pub struct ToolItemGroup(Object<ffi::GtkToolItemGroup, ffi::GtkToolItemGroupClass>) @extends Container, Widget, @implements Buildable, ToolShell;

    match fn {
        type_ => || ffi::gtk_tool_item_group_get_type(),
    }
}

impl ToolItemGroup {
    pub const NONE: Option<&'static ToolItemGroup> = None;

    #[doc(alias = "gtk_tool_item_group_new")]
    pub fn new(label: &str) -> ToolItemGroup {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_tool_item_group_new(label.to_glib_none().0))
                .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`ToolItemGroup`] objects.
    ///
    /// This method returns an instance of [`ToolItemGroupBuilder`](crate::builders::ToolItemGroupBuilder) which can be used to create [`ToolItemGroup`] objects.
    pub fn builder() -> ToolItemGroupBuilder {
        ToolItemGroupBuilder::new()
    }
}

impl Default for ToolItemGroup {
    fn default() -> Self {
        glib::object::Object::new::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`ToolItemGroup`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct ToolItemGroupBuilder {
    builder: glib::object::ObjectBuilder<'static, ToolItemGroup>,
}

impl ToolItemGroupBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn collapsed(self, collapsed: bool) -> Self {
        Self {
            builder: self.builder.property("collapsed", collapsed),
        }
    }

    pub fn ellipsize(self, ellipsize: pango::EllipsizeMode) -> Self {
        Self {
            builder: self.builder.property("ellipsize", ellipsize),
        }
    }

    pub fn header_relief(self, header_relief: ReliefStyle) -> Self {
        Self {
            builder: self.builder.property("header-relief", header_relief),
        }
    }

    pub fn label(self, label: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("label", label.into()),
        }
    }

    pub fn label_widget(self, label_widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self
                .builder
                .property("label-widget", label_widget.clone().upcast()),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ToolItemGroup`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> ToolItemGroup {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::ToolItemGroup>> Sealed for T {}
}

pub trait ToolItemGroupExt: IsA<ToolItemGroup> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_tool_item_group_get_collapsed")]
    #[doc(alias = "get_collapsed")]
    fn is_collapsed(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_collapsed(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_group_get_drop_item")]
    #[doc(alias = "get_drop_item")]
    fn drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_drop_item(
                self.as_ref().to_glib_none().0,
                x,
                y,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_group_get_ellipsize")]
    #[doc(alias = "get_ellipsize")]
    fn ellipsize(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_ellipsize(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_group_get_header_relief")]
    #[doc(alias = "get_header_relief")]
    fn header_relief(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_item_group_get_header_relief(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_group_get_item_position")]
    #[doc(alias = "get_item_position")]
    fn item_position(&self, item: &impl IsA<ToolItem>) -> i32 {
        unsafe {
            ffi::gtk_tool_item_group_get_item_position(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
            )
        }
    }

    #[doc(alias = "gtk_tool_item_group_get_label")]
    #[doc(alias = "get_label")]
    fn label(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_label(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_group_get_label_widget")]
    #[doc(alias = "get_label_widget")]
    fn label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_label_widget(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_group_get_n_items")]
    #[doc(alias = "get_n_items")]
    fn n_items(&self) -> u32 {
        unsafe { ffi::gtk_tool_item_group_get_n_items(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "gtk_tool_item_group_get_nth_item")]
    #[doc(alias = "get_nth_item")]
    fn nth_item(&self, index: u32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_item_group_get_nth_item(
                self.as_ref().to_glib_none().0,
                index,
            ))
        }
    }

    #[doc(alias = "gtk_tool_item_group_insert")]
    fn insert(&self, item: &impl IsA<ToolItem>, position: i32) {
        unsafe {
            ffi::gtk_tool_item_group_insert(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "gtk_tool_item_group_set_collapsed")]
    fn set_collapsed(&self, collapsed: bool) {
        unsafe {
            ffi::gtk_tool_item_group_set_collapsed(
                self.as_ref().to_glib_none().0,
                collapsed.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_group_set_ellipsize")]
    fn set_ellipsize(&self, ellipsize: pango::EllipsizeMode) {
        unsafe {
            ffi::gtk_tool_item_group_set_ellipsize(
                self.as_ref().to_glib_none().0,
                ellipsize.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_group_set_header_relief")]
    fn set_header_relief(&self, style: ReliefStyle) {
        unsafe {
            ffi::gtk_tool_item_group_set_header_relief(
                self.as_ref().to_glib_none().0,
                style.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_tool_item_group_set_item_position")]
    fn set_item_position(&self, item: &impl IsA<ToolItem>, position: i32) {
        unsafe {
            ffi::gtk_tool_item_group_set_item_position(
                self.as_ref().to_glib_none().0,
                item.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    #[doc(alias = "gtk_tool_item_group_set_label")]
    fn set_label(&self, label: &str) {
        unsafe {
            ffi::gtk_tool_item_group_set_label(
                self.as_ref().to_glib_none().0,
                label.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_tool_item_group_set_label_widget")]
    fn set_label_widget(&self, label_widget: &impl IsA<Widget>) {
        unsafe {
            ffi::gtk_tool_item_group_set_label_widget(
                self.as_ref().to_glib_none().0,
                label_widget.as_ref().to_glib_none().0,
            );
        }
    }

    fn item_expands<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "expand",
        )
    }

    fn set_item_expand<T: IsA<ToolItem>>(&self, item: &T, expand: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "expand",
            &expand,
        )
    }

    fn item_fills<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "fill",
        )
    }

    fn set_item_fill<T: IsA<ToolItem>>(&self, item: &T, fill: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "fill",
            &fill,
        )
    }

    fn item_is_homogeneous<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "homogeneous",
        )
    }

    fn set_item_homogeneous<T: IsA<ToolItem>>(&self, item: &T, homogeneous: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "homogeneous",
            &homogeneous,
        )
    }

    #[doc(alias = "item.new-row")]
    fn item_is_new_row<T: IsA<ToolItem>>(&self, item: &T) -> bool {
        crate::prelude::ContainerExtManual::child_property(
            self.as_ref(),
            &item.clone().upcast(),
            "new-row",
        )
    }

    #[doc(alias = "item.new-row")]
    fn set_item_new_row<T: IsA<ToolItem>>(&self, item: &T, new_row: bool) {
        crate::prelude::ContainerExtManual::child_set_property(
            self.as_ref(),
            &item.clone().upcast(),
            "new-row",
            &new_row,
        )
    }

    #[doc(alias = "collapsed")]
    fn connect_collapsed_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_collapsed_trampoline<
            P: IsA<ToolItemGroup>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItemGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItemGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::collapsed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_collapsed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "ellipsize")]
    fn connect_ellipsize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_ellipsize_trampoline<
            P: IsA<ToolItemGroup>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItemGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItemGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::ellipsize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_ellipsize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "header-relief")]
    fn connect_header_relief_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_header_relief_trampoline<
            P: IsA<ToolItemGroup>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItemGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItemGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::header-relief\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_header_relief_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "label")]
    fn connect_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_trampoline<P: IsA<ToolItemGroup>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkToolItemGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItemGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "label-widget")]
    fn connect_label_widget_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_label_widget_trampoline<
            P: IsA<ToolItemGroup>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolItemGroup,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(ToolItemGroup::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::label-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_label_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<ToolItemGroup>> ToolItemGroupExt for O {}

impl fmt::Display for ToolItemGroup {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ToolItemGroup")
    }
}

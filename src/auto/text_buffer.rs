// This file was generated by gir (a3f05e3) from gir-files (11e0e6d)
// DO NOT EDIT

use Clipboard;
use TextChildAnchor;
use TextIter;
use TextMark;
use TextTag;
use TextTagTable;
use ffi;
use ffi::GtkClipboard;
use ffi::GtkTextBuffer;
use ffi::GtkTextChildAnchor;
use ffi::GtkTextIter;
use ffi::GtkTextMark;
use ffi::GtkTextTag;
use gdk_pixbuf;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi::gpointer;
use libc::c_char;
use libc::c_int;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct TextBuffer(Object<ffi::GtkTextBuffer>);

    match fn {
        get_type => || ffi::gtk_text_buffer_get_type(),
    }
}

impl TextBuffer {
    pub fn new(table: Option<&TextTagTable>) -> TextBuffer {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_new(table.to_glib_none().0))
        }
    }

    pub fn add_mark(&self, mark: &TextMark, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_add_mark(self.to_glib_none().0, mark.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    pub fn add_selection_clipboard(&self, clipboard: &Clipboard) {
        unsafe {
            ffi::gtk_text_buffer_add_selection_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0);
        }
    }

    pub fn apply_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_apply_tag(self.to_glib_none().0, tag.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn apply_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_apply_tag_by_name(self.to_glib_none().0, name.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn backspace(&self, iter: &mut TextIter, interactive: bool, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_backspace(self.to_glib_none().0, iter.to_glib_none_mut().0, interactive.to_glib(), default_editable.to_glib()))
        }
    }

    pub fn begin_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_begin_user_action(self.to_glib_none().0);
        }
    }

    pub fn copy_clipboard(&self, clipboard: &Clipboard) {
        unsafe {
            ffi::gtk_text_buffer_copy_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0);
        }
    }

    pub fn create_child_anchor(&self, iter: &mut TextIter) -> Option<TextChildAnchor> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_create_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0))
        }
    }

    pub fn create_mark(&self, mark_name: Option<&str>, where_: &TextIter, left_gravity: bool) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_create_mark(self.to_glib_none().0, mark_name.to_glib_none().0, where_.to_glib_none().0, left_gravity.to_glib()))
        }
    }

    //pub fn create_tag(&self, tag_name: Option<&str>, first_property_name: Option<&str>, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<TextTag> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_create_tag() }
    //}

    pub fn cut_clipboard(&self, clipboard: &Clipboard, default_editable: bool) {
        unsafe {
            ffi::gtk_text_buffer_cut_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0, default_editable.to_glib());
        }
    }

    pub fn delete(&self, start: &mut TextIter, end: &mut TextIter) {
        unsafe {
            ffi::gtk_text_buffer_delete(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
        }
    }

    pub fn delete_interactive(&self, start_iter: &mut TextIter, end_iter: &mut TextIter, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_delete_interactive(self.to_glib_none().0, start_iter.to_glib_none_mut().0, end_iter.to_glib_none_mut().0, default_editable.to_glib()))
        }
    }

    pub fn delete_mark(&self, mark: &TextMark) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark(self.to_glib_none().0, mark.to_glib_none().0);
        }
    }

    pub fn delete_mark_by_name(&self, name: &str) {
        unsafe {
            ffi::gtk_text_buffer_delete_mark_by_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn delete_selection(&self, interactive: bool, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_delete_selection(self.to_glib_none().0, interactive.to_glib(), default_editable.to_glib()))
        }
    }

    //pub fn deserialize(&self, content_buffer: &TextBuffer, format: /*Ignored*/&gdk::Atom, iter: &mut TextIter, data: /*Unimplemented*/&CArray TypeId { ns_id: 0, id: 3 }, length: /*Unimplemented*/Fundamental: Size) -> Result<(), Error> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize() }
    //}

    //pub fn deserialize_get_can_create_tags(&self, format: /*Ignored*/&gdk::Atom) -> bool {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize_get_can_create_tags() }
    //}

    //pub fn deserialize_set_can_create_tags(&self, format: /*Ignored*/&gdk::Atom, can_create_tags: bool) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_deserialize_set_can_create_tags() }
    //}

    pub fn end_user_action(&self) {
        unsafe {
            ffi::gtk_text_buffer_end_user_action(self.to_glib_none().0);
        }
    }

    pub fn get_bounds(&self) -> (TextIter, TextIter) {
        unsafe {
            let mut start = TextIter::uninitialized();
            let mut end = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_bounds(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0);
            (start, end)
        }
    }

    pub fn get_char_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_char_count(self.to_glib_none().0)
        }
    }

    //pub fn get_copy_target_list(&self) -> /*Ignored*/Option<TargetList> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_copy_target_list() }
    //}

    //pub fn get_deserialize_formats(&self) -> (/*Ignored*/Vec<gdk::Atom>, i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_deserialize_formats() }
    //}

    pub fn get_end_iter(&self) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_end_iter(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn get_has_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_has_selection(self.to_glib_none().0))
        }
    }

    pub fn get_insert(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_insert(self.to_glib_none().0))
        }
    }

    pub fn get_iter_at_child_anchor(&self, anchor: &TextChildAnchor) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0, anchor.to_glib_none().0);
            iter
        }
    }

    pub fn get_iter_at_line(&self, line_number: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number);
            iter
        }
    }

    pub fn get_iter_at_line_index(&self, line_number: i32, byte_index: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line_index(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number, byte_index);
            iter
        }
    }

    pub fn get_iter_at_line_offset(&self, line_number: i32, char_offset: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_line_offset(self.to_glib_none().0, iter.to_glib_none_mut().0, line_number, char_offset);
            iter
        }
    }

    pub fn get_iter_at_mark(&self, mark: &TextMark) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_mark(self.to_glib_none().0, iter.to_glib_none_mut().0, mark.to_glib_none().0);
            iter
        }
    }

    pub fn get_iter_at_offset(&self, char_offset: i32) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_iter_at_offset(self.to_glib_none().0, iter.to_glib_none_mut().0, char_offset);
            iter
        }
    }

    pub fn get_line_count(&self) -> i32 {
        unsafe {
            ffi::gtk_text_buffer_get_line_count(self.to_glib_none().0)
        }
    }

    pub fn get_mark(&self, name: &str) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_mark(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    pub fn get_modified(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_get_modified(self.to_glib_none().0))
        }
    }

    //pub fn get_paste_target_list(&self) -> /*Ignored*/Option<TargetList> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_paste_target_list() }
    //}

    pub fn get_selection_bound(&self) -> Option<TextMark> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_selection_bound(self.to_glib_none().0))
        }
    }

    pub fn get_selection_bounds(&self) -> Option<(TextIter, TextIter)> {
        unsafe {
            let mut start = TextIter::uninitialized();
            let mut end = TextIter::uninitialized();
            let ret = from_glib(ffi::gtk_text_buffer_get_selection_bounds(self.to_glib_none().0, start.to_glib_none_mut().0, end.to_glib_none_mut().0));
            if ret { Some((start, end)) } else { None }
        }
    }

    //pub fn get_serialize_formats(&self) -> (/*Ignored*/Vec<gdk::Atom>, i32) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_get_serialize_formats() }
    //}

    pub fn get_slice(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_get_slice(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0, include_hidden_chars.to_glib()))
        }
    }

    pub fn get_start_iter(&self) -> TextIter {
        unsafe {
            let mut iter = TextIter::uninitialized();
            ffi::gtk_text_buffer_get_start_iter(self.to_glib_none().0, iter.to_glib_none_mut().0);
            iter
        }
    }

    pub fn get_tag_table(&self) -> Option<TextTagTable> {
        unsafe {
            from_glib_none(ffi::gtk_text_buffer_get_tag_table(self.to_glib_none().0))
        }
    }

    pub fn get_text(&self, start: &TextIter, end: &TextIter, include_hidden_chars: bool) -> Option<String> {
        unsafe {
            from_glib_full(ffi::gtk_text_buffer_get_text(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0, include_hidden_chars.to_glib()))
        }
    }

    pub fn insert_child_anchor(&self, iter: &mut TextIter, anchor: &TextChildAnchor) {
        unsafe {
            ffi::gtk_text_buffer_insert_child_anchor(self.to_glib_none().0, iter.to_glib_none_mut().0, anchor.to_glib_none().0);
        }
    }

    pub fn insert_pixbuf(&self, iter: &mut TextIter, pixbuf: &gdk_pixbuf::Pixbuf) {
        unsafe {
            ffi::gtk_text_buffer_insert_pixbuf(self.to_glib_none().0, iter.to_glib_none_mut().0, pixbuf.to_glib_none().0);
        }
    }

    pub fn insert_range(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_insert_range(self.to_glib_none().0, iter.to_glib_none_mut().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn insert_range_interactive(&self, iter: &mut TextIter, start: &TextIter, end: &TextIter, default_editable: bool) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_buffer_insert_range_interactive(self.to_glib_none().0, iter.to_glib_none_mut().0, start.to_glib_none().0, end.to_glib_none().0, default_editable.to_glib()))
        }
    }

    //pub fn insert_with_tags(&self, iter: &mut TextIter, text: &str, len: i32, first_tag: &TextTag, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags() }
    //}

    //pub fn insert_with_tags_by_name(&self, iter: &mut TextIter, text: &str, len: i32, first_tag_name: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_insert_with_tags_by_name() }
    //}

    pub fn move_mark(&self, mark: &TextMark, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_move_mark(self.to_glib_none().0, mark.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    pub fn move_mark_by_name(&self, name: &str, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_move_mark_by_name(self.to_glib_none().0, name.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    pub fn paste_clipboard(&self, clipboard: &Clipboard, override_location: Option<&TextIter>, default_editable: bool) {
        unsafe {
            ffi::gtk_text_buffer_paste_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0, mut_override(override_location.to_glib_none().0), default_editable.to_glib());
        }
    }

    pub fn place_cursor(&self, where_: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_place_cursor(self.to_glib_none().0, where_.to_glib_none().0);
        }
    }

    //pub fn register_deserialize_format(&self, mime_type: &str, function: /*Unknown conversion*//*Unimplemented*/TextBufferDeserializeFunc, user_data: /*Unimplemented*/Fundamental: Pointer, user_data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> /*Ignored*/Option<gdk::Atom> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_deserialize_format() }
    //}

    //pub fn register_deserialize_tagset(&self, tagset_name: Option<&str>) -> /*Ignored*/Option<gdk::Atom> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_deserialize_tagset() }
    //}

    //pub fn register_serialize_format(&self, mime_type: &str, function: /*Unknown conversion*//*Unimplemented*/TextBufferSerializeFunc, user_data: /*Unimplemented*/Fundamental: Pointer, user_data_destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) -> /*Ignored*/Option<gdk::Atom> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_serialize_format() }
    //}

    //pub fn register_serialize_tagset(&self, tagset_name: Option<&str>) -> /*Ignored*/Option<gdk::Atom> {
    //    unsafe { TODO: call ffi::gtk_text_buffer_register_serialize_tagset() }
    //}

    pub fn remove_all_tags(&self, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_all_tags(self.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn remove_selection_clipboard(&self, clipboard: &Clipboard) {
        unsafe {
            ffi::gtk_text_buffer_remove_selection_clipboard(self.to_glib_none().0, clipboard.to_glib_none().0);
        }
    }

    pub fn remove_tag(&self, tag: &TextTag, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_tag(self.to_glib_none().0, tag.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn remove_tag_by_name(&self, name: &str, start: &TextIter, end: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_remove_tag_by_name(self.to_glib_none().0, name.to_glib_none().0, start.to_glib_none().0, end.to_glib_none().0);
        }
    }

    pub fn select_range(&self, ins: &TextIter, bound: &TextIter) {
        unsafe {
            ffi::gtk_text_buffer_select_range(self.to_glib_none().0, ins.to_glib_none().0, bound.to_glib_none().0);
        }
    }

    //pub fn serialize(&self, content_buffer: &TextBuffer, format: /*Ignored*/&gdk::Atom, start: &TextIter, end: &TextIter) -> (/*Unimplemented*/CArray TypeId { ns_id: 0, id: 3 }, /*Unimplemented*/Fundamental: Size) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_serialize() }
    //}

    pub fn set_modified(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_buffer_set_modified(self.to_glib_none().0, setting.to_glib());
        }
    }

    //pub fn unregister_deserialize_format(&self, format: /*Ignored*/&gdk::Atom) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_unregister_deserialize_format() }
    //}

    //pub fn unregister_serialize_format(&self, format: /*Ignored*/&gdk::Atom) {
    //    unsafe { TODO: call ffi::gtk_text_buffer_unregister_serialize_format() }
    //}

    pub fn connect_apply_tag<F: Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "apply-tag",
                transmute(apply_tag_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_begin_user_action<F: Fn(&TextBuffer) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "begin-user-action",
                transmute(begin_user_action_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_changed<F: Fn(&TextBuffer) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_delete_range<F: Fn(&TextBuffer, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextIter, &TextIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "delete-range",
                transmute(delete_range_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_end_user_action<F: Fn(&TextBuffer) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "end-user-action",
                transmute(end_user_action_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_insert_child_anchor<F: Fn(&TextBuffer, &TextIter, &TextChildAnchor) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextIter, &TextChildAnchor) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-child-anchor",
                transmute(insert_child_anchor_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_insert_text<F: Fn(&TextBuffer, &TextIter, &str, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextIter, &str, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert-text",
                transmute(insert_text_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_mark_deleted<F: Fn(&TextBuffer, &TextMark) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextMark) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "mark-deleted",
                transmute(mark_deleted_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_mark_set<F: Fn(&TextBuffer, &TextIter, &TextMark) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextIter, &TextMark) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "mark-set",
                transmute(mark_set_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_modified_changed<F: Fn(&TextBuffer) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "modified-changed",
                transmute(modified_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_paste_done<F: Fn(&TextBuffer, &Clipboard) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &Clipboard) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "paste-done",
                transmute(paste_done_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    pub fn connect_remove_tag<F: Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "remove-tag",
                transmute(remove_tag_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn apply_tag_trampoline(this: *mut GtkTextBuffer, tag: *mut GtkTextTag, start: *mut GtkTextIter, end: *mut GtkTextIter, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(tag), &from_glib_none(start), &from_glib_none(end))
}

unsafe extern "C" fn begin_user_action_trampoline(this: *mut GtkTextBuffer, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn changed_trampoline(this: *mut GtkTextBuffer, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn delete_range_trampoline(this: *mut GtkTextBuffer, start: *mut GtkTextIter, end: *mut GtkTextIter, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextIter, &TextIter) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(start), &from_glib_none(end))
}

unsafe extern "C" fn end_user_action_trampoline(this: *mut GtkTextBuffer, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn insert_child_anchor_trampoline(this: *mut GtkTextBuffer, location: *mut GtkTextIter, anchor: *mut GtkTextChildAnchor, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextIter, &TextChildAnchor) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(location), &from_glib_none(anchor))
}

unsafe extern "C" fn insert_text_trampoline(this: *mut GtkTextBuffer, location: *mut GtkTextIter, text: *mut c_char, len: c_int, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextIter, &str, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(location), &String::from_glib_none(text), len)
}

unsafe extern "C" fn mark_deleted_trampoline(this: *mut GtkTextBuffer, mark: *mut GtkTextMark, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextMark) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(mark))
}

unsafe extern "C" fn mark_set_trampoline(this: *mut GtkTextBuffer, location: *mut GtkTextIter, mark: *mut GtkTextMark, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextIter, &TextMark) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(location), &from_glib_none(mark))
}

unsafe extern "C" fn modified_changed_trampoline(this: *mut GtkTextBuffer, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

unsafe extern "C" fn paste_done_trampoline(this: *mut GtkTextBuffer, clipboard: *mut GtkClipboard, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &Clipboard) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(clipboard))
}

unsafe extern "C" fn remove_tag_trampoline(this: *mut GtkTextBuffer, tag: *mut GtkTextTag, start: *mut GtkTextIter, end: *mut GtkTextIter, f: gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&TextBuffer, &TextTag, &TextIter, &TextIter) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(tag), &from_glib_none(start), &from_glib_none(end))
}

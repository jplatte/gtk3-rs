// This file was generated by gir (f00d658) from gir-files (0bcaef9)
// DO NOT EDIT

use Error;
use ResourceLookupFlags;
use ffi;
use glib;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Resource(Shared<ffi::GResource>);

    match fn {
        ref => |ptr| ffi::g_resource_ref(ptr),
        unref => |ptr| ffi::g_resource_unref(ptr),
        get_type => || ffi::g_resource_get_type(),
    }
}

impl Resource {
    pub fn new_from_data(data: &glib::Bytes) -> Result<Resource, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_new_from_data(data.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn enumerate_children(&self, path: &str, lookup_flags: ResourceLookupFlags) -> Result<Vec<String>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_enumerate_children(self.to_glib_none().0, path.to_glib_none().0, lookup_flags.to_glib(), &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_info(&self, path: &str, lookup_flags: ResourceLookupFlags) -> Result<(usize, u32), Error> {
        unsafe {
            let mut size = mem::uninitialized();
            let mut flags = mem::uninitialized();
            let mut error = ptr::null_mut();
            let _ = ffi::g_resource_get_info(self.to_glib_none().0, path.to_glib_none().0, lookup_flags.to_glib(), &mut size, &mut flags, &mut error);
            if error.is_null() { Ok((size, flags)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn lookup_data(&self, path: &str, lookup_flags: ResourceLookupFlags) -> Result<glib::Bytes, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_lookup_data(self.to_glib_none().0, path.to_glib_none().0, lookup_flags.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    //pub fn open_stream(&self, path: &str, lookup_flags: ResourceLookupFlags) -> Result</*Ignored*/InputStream, Error> {
    //    unsafe { TODO: call ffi::g_resource_open_stream() }
    //}

    pub fn load<P: AsRef<std::path::Path>>(filename: P) -> Result<Resource, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_resource_load(filename.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Pixbuf;
use crate::PixbufAnimation;
use glib::object::Cast;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GdkPixbufNonAnim")]
    pub struct PixbufNonAnim(Object<ffi::GdkPixbufNonAnim>) @extends PixbufAnimation;

    match fn {
        type_ => || ffi::gdk_pixbuf_non_anim_get_type(),
    }
}

impl PixbufNonAnim {
    #[doc(alias = "gdk_pixbuf_non_anim_new")]
    pub fn new(pixbuf: &Pixbuf) -> PixbufNonAnim {
        unsafe {
            PixbufAnimation::from_glib_full(ffi::gdk_pixbuf_non_anim_new(pixbuf.to_glib_none().0))
                .unsafe_cast()
        }
    }
}

impl fmt::Display for PixbufNonAnim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PixbufNonAnim")
    }
}
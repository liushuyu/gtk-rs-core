// This file was generated by gir (f8c137b) from gir-files (11e0e6d)
// DO NOT EDIT

use Box;
use Buildable;
use ButtonBoxStyle;
use Container;
use Orientable;
use Orientation;
use Widget;
use ffi;
use glib::translate::*;
use glib::types;
use object::*;

pub type ButtonBox = Object<ffi::GtkButtonBox>;

unsafe impl Upcast<Widget> for ButtonBox { }
unsafe impl Upcast<Container> for ButtonBox { }
unsafe impl Upcast<Box> for ButtonBox { }
unsafe impl Upcast<Buildable> for ButtonBox { }
unsafe impl Upcast<Orientable> for ButtonBox { }

impl ButtonBox {
    pub fn new(orientation: Orientation) -> ButtonBox {
        unsafe {
            Widget::from_glib_none(ffi::gtk_button_box_new(orientation)).downcast_unchecked()
        }
    }

    pub fn get_child_non_homogeneous<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_non_homogeneous(self.to_glib_none().0, child.upcast().to_glib_none().0))
        }
    }

    pub fn get_child_secondary<T: Upcast<Widget>>(&self, child: &T) -> bool {
        unsafe {
            from_glib(ffi::gtk_button_box_get_child_secondary(self.to_glib_none().0, child.upcast().to_glib_none().0))
        }
    }

    pub fn get_layout(&self) -> ButtonBoxStyle {
        unsafe {
            ffi::gtk_button_box_get_layout(self.to_glib_none().0)
        }
    }

    pub fn set_child_non_homogeneous<T: Upcast<Widget>>(&self, child: &T, non_homogeneous: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_non_homogeneous(self.to_glib_none().0, child.upcast().to_glib_none().0, non_homogeneous.to_glib());
        }
    }

    pub fn set_child_secondary<T: Upcast<Widget>>(&self, child: &T, is_secondary: bool) {
        unsafe {
            ffi::gtk_button_box_set_child_secondary(self.to_glib_none().0, child.upcast().to_glib_none().0, is_secondary.to_glib());
        }
    }

    pub fn set_layout(&self, layout_style: ButtonBoxStyle) {
        unsafe {
            ffi::gtk_button_box_set_layout(self.to_glib_none().0, layout_style);
        }
    }

}

impl types::StaticType for ButtonBox {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_button_box_get_type()) }
    }
}

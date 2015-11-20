// This file was generated by gir (f8c137b) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
#[cfg(gtk_3_6)]
use ArrowType;
use Bin;
use Buildable;
use Button;
use Container;
#[cfg(gtk_3_6)]
use Menu;
#[cfg(gtk_3_12)]
use Popover;
use ToggleButton;
use Widget;
use ffi;
use glib::translate::*;
use glib::types;
use object::*;

pub type MenuButton = Object<ffi::GtkMenuButton>;

unsafe impl Upcast<Widget> for MenuButton { }
unsafe impl Upcast<Container> for MenuButton { }
unsafe impl Upcast<Bin> for MenuButton { }
unsafe impl Upcast<Button> for MenuButton { }
unsafe impl Upcast<ToggleButton> for MenuButton { }
unsafe impl Upcast<Actionable> for MenuButton { }
unsafe impl Upcast<Buildable> for MenuButton { }

impl MenuButton {
    #[cfg(gtk_3_6)]
    pub fn new() -> MenuButton {
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_button_new()).downcast_unchecked()
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_align_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_align_widget(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_direction(&self) -> ArrowType {
        unsafe {
            ffi::gtk_menu_button_get_direction(self.to_glib_none().0)
        }
    }

    //#[cfg(gtk_3_6)]
    //pub fn get_menu_model(&self) -> /*Ignored*/Option<gio::MenuModel> {
    //    unsafe { TODO: call ffi::gtk_menu_button_get_menu_model() }
    //}

    #[cfg(gtk_3_12)]
    pub fn get_popover(&self) -> Option<Popover> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popover(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_6)]
    pub fn get_popup(&self) -> Option<Menu> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popup(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_12)]
    pub fn get_use_popover(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_use_popover(self.to_glib_none().0))
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_align_widget<T: Upcast<Widget>>(&self, align_widget: Option<&T>) {
        unsafe {
            ffi::gtk_menu_button_set_align_widget(self.to_glib_none().0, align_widget.map(Upcast::upcast).to_glib_none().0);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_direction(&self, direction: ArrowType) {
        unsafe {
            ffi::gtk_menu_button_set_direction(self.to_glib_none().0, direction);
        }
    }

    //#[cfg(gtk_3_6)]
    //pub fn set_menu_model<T: Upcast</*Ignored*/gio::MenuModel>>(&self, menu_model: Option<&T>) {
    //    unsafe { TODO: call ffi::gtk_menu_button_set_menu_model() }
    //}

    #[cfg(gtk_3_12)]
    pub fn set_popover<T: Upcast<Widget>>(&self, popover: Option<&T>) {
        unsafe {
            ffi::gtk_menu_button_set_popover(self.to_glib_none().0, popover.map(Upcast::upcast).to_glib_none().0);
        }
    }

    #[cfg(gtk_3_6)]
    pub fn set_popup<T: Upcast<Widget>>(&self, menu: Option<&T>) {
        unsafe {
            ffi::gtk_menu_button_set_popup(self.to_glib_none().0, menu.map(Upcast::upcast).to_glib_none().0);
        }
    }

    #[cfg(gtk_3_12)]
    pub fn set_use_popover(&self, use_popover: bool) {
        unsafe {
            ffi::gtk_menu_button_set_use_popover(self.to_glib_none().0, use_popover.to_glib());
        }
    }

}

impl types::StaticType for MenuButton {
    #[inline]
    fn static_type() -> types::Type {
        unsafe { from_glib(ffi::gtk_menu_button_get_type()) }
    }
}

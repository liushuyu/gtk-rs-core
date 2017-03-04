// This file was generated by gir (c3b4020) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
#[cfg(feature = "v3_6")]
use ArrowType;
use Bin;
use Button;
use Container;
#[cfg(feature = "v3_6")]
use Menu;
#[cfg(feature = "v3_12")]
use Popover;
use ToggleButton;
use Widget;
use ffi;
#[cfg(feature = "v3_6")]
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct MenuButton(Object<ffi::GtkMenuButton>): ToggleButton, Button, Bin, Container, Widget, Actionable;

    match fn {
        get_type => || ffi::gtk_menu_button_get_type(),
    }
}

impl MenuButton {
    #[cfg(feature = "v3_6")]
    pub fn new() -> MenuButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_menu_button_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_align_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_align_widget(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_direction(&self) -> ArrowType {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_direction(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_menu_model(&self) -> Option<gio::MenuModel> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_menu_model(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_popover(&self) -> Option<Popover> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popover(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn get_popup(&self) -> Option<Menu> {
        unsafe {
            from_glib_none(ffi::gtk_menu_button_get_popup(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn get_use_popover(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_button_get_use_popover(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_align_widget<T: IsA<Widget>>(&self, align_widget: Option<&T>) {
        unsafe {
            ffi::gtk_menu_button_set_align_widget(self.to_glib_none().0, align_widget.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_direction(&self, direction: ArrowType) {
        unsafe {
            ffi::gtk_menu_button_set_direction(self.to_glib_none().0, direction.to_glib());
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_menu_model<T: IsA<gio::MenuModel>>(&self, menu_model: Option<&T>) {
        unsafe {
            ffi::gtk_menu_button_set_menu_model(self.to_glib_none().0, menu_model.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_popover<T: IsA<Widget>>(&self, popover: Option<&T>) {
        unsafe {
            ffi::gtk_menu_button_set_popover(self.to_glib_none().0, popover.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_6")]
    pub fn set_popup<T: IsA<Widget>>(&self, menu: Option<&T>) {
        unsafe {
            ffi::gtk_menu_button_set_popup(self.to_glib_none().0, menu.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    pub fn set_use_popover(&self, use_popover: bool) {
        unsafe {
            ffi::gtk_menu_button_set_use_popover(self.to_glib_none().0, use_popover.to_glib());
        }
    }
}

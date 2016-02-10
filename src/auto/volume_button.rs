// This file was generated by gir (17af302) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Orientable;
use ScaleButton;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct VolumeButton(Object<ffi::GtkVolumeButton>): Widget, Container, Bin, Button, ScaleButton, Actionable, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_volume_button_get_type(),
    }
}

impl VolumeButton {
    pub fn new() -> VolumeButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_volume_button_new()).downcast_unchecked()
        }
    }
}

// This file was generated by gir (af5277e) from gir-files (11e0e6d)
// DO NOT EDIT

use AppChooser;
use Bin;
use Container;
use Dialog;
use DialogFlags;
use Widget;
use Window;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct AppChooserDialog(Object<ffi::GtkAppChooserDialog>): Dialog, Window, Bin, Container, Widget, AppChooser;

    match fn {
        get_type => || ffi::gtk_app_chooser_dialog_get_type(),
    }
}

impl AppChooserDialog {
    //pub fn new<T: IsA<Window>, U: IsA</*Ignored*/gio::File>>(parent: Option<&T>, flags: DialogFlags, file: &U) -> AppChooserDialog {
    //    unsafe { TODO: call ffi::gtk_app_chooser_dialog_new() }
    //}

    pub fn new_for_content_type<T: IsA<Window>>(parent: Option<&T>, flags: DialogFlags, content_type: &str) -> AppChooserDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_dialog_new_for_content_type(parent.to_glib_none().0, flags.to_glib(), content_type.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_heading(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_heading(self.to_glib_none().0))
        }
    }

    pub fn get_widget(&self) -> Widget {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_dialog_get_widget(self.to_glib_none().0))
        }
    }

    pub fn set_heading(&self, heading: &str) {
        unsafe {
            ffi::gtk_app_chooser_dialog_set_heading(self.to_glib_none().0, heading.to_glib_none().0);
        }
    }
}

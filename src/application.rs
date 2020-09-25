
//! # Builder Basics Sample
//!
//! This sample demonstrates how to use the builder with an imported glade file

extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use gtk::{ApplicationWindow, Builder, Button, MessageDialog};

use std::env::args;

// Build the app ui
pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("app.glade");
    let builder = Builder::from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("window1").expect("Couldn't get window1");
    window.set_application(Some(application));
    let bigbutton: Button = builder.get_object("button1").expect("Couldn't get button1");
    
    
    // dialog
    let dialog: MessageDialog = builder
        .get_object("messagedialog1")
        .expect("Couldn't get messagedialog1");

    dialog.connect_delete_event(|dialog, _| {
        dialog.hide();
        gtk::Inhibit(true)
    });

    bigbutton.connect_clicked(clone!(@weak dialog => move |_| dialog.show_all()));
    
    window.show_all();
}

pub fn app() {
    // create new app
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.screenshot"),
        Default::default(),
    )
    .expect("Initialization failed...");

    // when application is lanched
    application.connect_activate(|app| {
        build_ui(app);
    });
    // run the Application
    let exit_code = application.run(&args().collect::<Vec<_>>());
    std::process::exit(exit_code);
    
}
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{
    ApplicationWindow, Builder, Button, ListBoxRow, RadioButton, SpinButton, Switch, Widget,
};
use std::env::args;

const APP_NAME: &str = "com.github.gtk-rs.examples.screenshot";
const DISABLE: bool = false;
const ENABLE: bool = true;

// Build the app ui
pub fn build_ui(application: &gtk::Application) {
    // Builder the ui from an xml builder string
    let builder = Builder::from_string(include_str!("app.ui"));
    // get object references from builder
    let window: ApplicationWindow = builder
        .get_object("appwindow")
        .expect("Couldn't get appwindow from builder");
    let capture_button: Button = builder
        .get_object("capture_button")
        .expect("Couldn't get capture_button from builder");
    let screen_button: RadioButton = builder
        .get_object("screen_button")
        .expect("Couldn't get screen_button from builder");
    let window_button: RadioButton = builder
        .get_object("window_button")
        .expect("Couldn't get window_button from builder");
    let selection_button: RadioButton = builder
        .get_object("selection_button")
        .expect("Couldn't get selection_button from builder");
    let show_pointer_switch: Switch = builder
        .get_object("show_pointer_switch")
        .expect("Couldn't get show_pointer_switch from builder");
    let delay_spiner: SpinButton = builder
        .get_object("delay_spiner")
        .expect("Couldn't get delay_spiner from builder");
    let pointer_row: ListBoxRow = builder
        .get_object("pointer_row")
        .expect("Could't get pointer_row from builder");
    // set app to the builder appwindow reference
    window.set_application(Some(application));

    //create action
    let action_capture_buttion = gio::SimpleAction::new("capture_buttion", None);
    // do this when the action is called
    action_capture_buttion.connect_activate(clone!(@weak show_pointer_switch, @weak delay_spiner,@weak screen_button,@weak window_button,@weak selection_button,@weak application => move |_,_| {
            println!("capture button presseed");
            //get capture mode
            println!("screen_button state is {}",screen_button.get_active());
            println!("window_button state is {}",window_button.get_active());
            println!("selection_button state is {}",selection_button.get_active());
            // get show pointer state
            println!("show_pointer_button state is {}", show_pointer_switch.get_state());
            // get display number in seconds
            println!("display_spinner value is {}", delay_spiner.get_value());
            //
            screenshot( &application);
        }),
    );
    // add the action to the window
    window.add_action(&action_capture_buttion);
    // connect the action to the buttun
    capture_button.connect_clicked(clone!(@weak action_capture_buttion => move |_| {
        action_capture_buttion.activate(None);  // use the action
    }));

    // mouse clicked signal
    screen_button.connect_clicked(clone!(@weak pointer_row => move |_|{
        println!("screen radio button pressed");
        pointer_row.set_sensitive(ENABLE);
    }));
    // mouse clicked signal
    window_button.connect_clicked(clone!(@weak pointer_row => move |_| {
        println!("window radio button pressed");
        pointer_row.set_sensitive(ENABLE);
    }));
    // mouse clicked signal
    selection_button.connect_clicked(clone!(@weak pointer_row => move  |_| {
        println!("selection radio button pressed");
        pointer_row.set_sensitive(DISABLE);
    }));
    // Display the widgets in the window
    window.show_all();
}

pub fn new() {
    // create new app
    let application = gtk::Application::new(Some(APP_NAME), gio::ApplicationFlags::default())
        .expect("Initialization failed...");

    // when application is lanched
    application.connect_activate(|app| {
        build_ui(app);
    });
    // run the Application
    let exit_code = application.run(&args().collect::<Vec<_>>());
    std::process::exit(exit_code);
}

pub fn screenshot(application: &gtk::Application) {
    let bus = application.get_dbus_connection().unwrap();
    // Plain call
    let result = bus.call_future(
        Some("org.gnome.Shell.Screenshot"), // bus name
        "/org/gnome/Shell/Screenshot",      // object path
        "org.gnome.Shell.Screenshot",       // interface name
        "Screenshot",                       // method name
        None,                               //
        None,                               //
        gio::DBusCallFlags::NONE,           //
        69,                                 // timeout msec
    );
    //  eprintln!("{:?}", result);
}

// method_name = "Screenshot";
//       method_params = g_variant_new ("(bbs)",
//                                      screenshot_config->include_pointer,
//                                      TRUE, /* flash */
//                                      filename);
//     }

//   connection = g_application_get_dbus_connection (g_application_get_default ());
//   g_dbus_connection_call_sync (connection,
//                                "org.gnome.Shell.Screenshot",
//                                "/org/gnome/Shell/Screenshot",
//                                "org.gnome.Shell.Screenshot",
//                                method_name,
//                                method_params,
//                                NULL,
//                                G_DBUS_CALL_FLAGS_NONE,
//                                -1,
//                                NULL,
//                                &error);

//   if (error == NULL)
//     {
//       screenshot = gdk_pixbuf_new_from_file (filename, &error);

//       /* remove the temporary file created by the shell */
//       g_unlink (filename);
//     }

//   return screenshot;
// }


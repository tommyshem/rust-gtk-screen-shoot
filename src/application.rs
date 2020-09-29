use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, RadioButton, SpinButton, Switch};
use std::env::args;

const APP_NAME: &str = "com.github.gtk-rs.examples.screenshot";

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
    // set app to the builder appwindow reference
    window.set_application(Some(application));

    // connect signals
    capture_button.connect_clicked(move|_| 
        {println!("capture button presseed");
        // get show pointer state
        println!("state is {}",show_pointer_switch.get_state());
        // get display number in seconds
        println!("value is {}",delay_spiner.get_value());
    }
    );
    screen_button.connect_clicked(|button| println!("screen radio button pressed"));
    window_button.connect_clicked(|button| println!("window radio button pressed"));

    selection_button.connect_clicked(move|_| {
        println!("selection radio button pressed");
        //show_pointer_switch.visible(false);
    });

    //= false; // .connect_clicked (|button| println!("show pointer switch pressed"));
    //delay_spiner::active(false); // connect_clicked(|button| println!("delay spiner pressed");
    // Display the widgets in the window
    window.show_all();
}

pub fn app() {
    // create new app
    let application = gtk::Application::new(Some(APP_NAME), Default::default())
        .expect("Initialization failed...");

    // when application is lanched
    application.connect_activate(|app| {
        build_ui(app);
    });
    // run the Application
    let exit_code = application.run(&args().collect::<Vec<_>>());
    std::process::exit(exit_code);
}

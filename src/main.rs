mod cli;
mod application;
extern crate gtk;
extern crate glib;


// Main entry point
fn main() {
    cli::get_arguments_passed_in();
    application::app();
}


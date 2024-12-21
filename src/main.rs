use gtk::{glib, prelude::*};

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("pro.lasne.demo-app")
        .build();
    application.connect_activate(build_ui);
    application.run()
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title(Some("Hello world"));
    window.set_default_size(200, 70);

    let button = gtk::Button::with_label("Click me!");
    button.connect_clicked(|_| {
        println!("You clicked me !");
    });

    window.set_child(Some(&button));
    window.present();
}
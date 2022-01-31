use gtk::prelude::*;
use gtk::glib;
use gtk::{
    Application,
    AboutDialog, 
    AccelFlags, 
    AccelGroup, 
    ApplicationWindow, 
    CheckMenuItem, 
    IconSize, 
    Image, 
    Label,
    Menu, 
    MenuBar, 
    MenuItem, 
    WindowPosition,
};




fn main() {

    let application = Application::builder()
        .application_id("org.rowechelon.RowEchelon")
        .build();

    application.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("RowEchelon")
            .default_width(300)
            .default_height(500)
            .build();

        window.show_all();
    });

    application.run();
}

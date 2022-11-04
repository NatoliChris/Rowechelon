mod entity;
mod ui;



use gtk::prelude::*;
use gtk::{
    Application,
    ApplicationWindow, 
    TreeView,
    TreeStore
};


fn main() {

    let application = Application::builder()
        .application_id("org.rowechelon.RowEchelon")
        .build();

    application.connect_activate(|app| {
        // Load CSS from ./style/style.css for now
        let provider = gtk::CssProvider::new();
        let style = include_bytes!("style/style.css");
        provider.load_from_data(style).expect("Failed to load CSS");

        gtk::StyleContext::add_provider_for_screen(
            &gtk::gdk::Screen::default().expect("Error initializing gtk css provider"),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let window = ApplicationWindow::builder()
            .application(app)
            .title("RowEchelon")
            .default_width(300)
            .default_height(500)
            .build();

        window.set_position(gtk::WindowPosition::Center);
        
        let topbox = gtk::Box::new(gtk::Orientation::Horizontal, 3); 
        let bottombox = gtk::Box::new(gtk:: Orientation::Horizontal, 3);
        let button = gtk::Button::with_label("A");
        button.set_size_request(48, 48);
        button.set_margin_bottom(0);
        button.set_border_width(0);
        
        let searchbar = gtk::SearchBar::new();
        searchbar.set_search_mode(true);
        searchbar.set_search_mode_enabled(true);
        topbox.add(&button);
        topbox.set_margin_top(1);
        topbox.set_margin_start(1);
        topbox.set_margin_end(1);

        let endbutton = gtk::Button::with_label("-");
        button.set_size_request(48, 48);
        button.set_margin_bottom(1);
        button.set_border_width(0);

        topbox.add(&searchbar);
        topbox.add(&endbutton);

        bottombox.set_hexpand(true);
        let leftbutton = gtk::Button::with_label("S");
        leftbutton.set_size_request(48, 48);
        leftbutton.set_margin(1);
        //leftbutton.set_hexpand(true);
        leftbutton.set_margin_start(30); 
        leftbutton.set_halign(gtk::Align::Start);

        let centrebutton = gtk::Button::with_label("J");
        centrebutton.set_size_request(48, 48);
        centrebutton.set_margin(1);
        centrebutton.set_halign(gtk::Align::Center);
        // Can add markup to tooltips?!
        centrebutton.set_tooltip_markup(Some(
            "<span foreground=\"#AABBCC\" size=\"x-large\">Join?</span>",
        ));
        //centrebutton.set_margin_start(30);

        // Join button
        let endbutton = gtk::Button::with_label("+");
        endbutton.set_size_request(48, 48);
        endbutton.set_margin(1);
        endbutton.set_margin_end(30);
        endbutton.set_halign(gtk::Align::End);
        // Attempt to add a css id to the button
        endbutton.set_widget_name("plusbutton");
        // What about tooltips?
        endbutton.set_tooltip_text(Some("New Message"));
        bottombox.pack_start(&leftbutton, true, true, 0);
        bottombox.add(&centrebutton);
        bottombox.pack_end(&endbutton, true, true, 0);
        bottombox.set_border_width(1);
        let treeview = TreeView::new();
        let treestore = TreeStore::new(&[String::static_type(),
            String::static_type()]);
        treeview.set_model(Some(&treestore));
        
        endbutton.connect_clicked(|_| {
            let window = gtk::Window::new(gtk::WindowType::Toplevel);
            window.set_title("ChatWindow");
            window.set_default_size(500, 400);
            let layout = gtk::Box::new(gtk::Orientation::Vertical, 4);
            let tabbox = gtk::Box::new(gtk::Orientation::Horizontal, 1);
            tabbox.set_visible(false); //Not in use right now
            let headerbox = gtk::Box::new(gtk::Orientation::Vertical, 2);
            let main_box = gtk::Box::new(gtk::Orientation::Horizontal, 1);
            let response_box = gtk::Box::new(gtk::Orientation::Horizontal, 2);

            let header_text = gtk::Label::new(Some("Header"));
            let topic_text = gtk::Label::new(Some("Topic"));
            
            //Hahaha what is this hack...
            topic_text.set_markup("<small>Topic</small>");

            headerbox.add(&header_text);
            headerbox.add(&topic_text);

            main_box.set_expand(true);
           

            let textview = gtk::TextView::new();
            main_box.add(&textview);
            
            textview.set_cursor_visible(false);
            textview.set_editable(false);
            textview.set_hexpand(true);

            
            let responseview = gtk::TextView::new();
            response_box.add(&responseview);
            
            responseview.set_hexpand(true);
            let sendbtn = gtk::Button::with_label(">");
            sendbtn.set_size_request(48, 48);
            response_box.add(&sendbtn);

            layout.add(&tabbox);
            layout.add(&headerbox);
            layout.add(&main_box);
            layout.add(&response_box);
            window.add(&layout);
            window.show_all();
        });


            
        let iter = treestore.insert_with_values(None, None, 
                                     &[(0, &"One"), 
                                     (1, &"A")]);
        treestore.insert_with_values(Some(&iter), None, 
                                     &[(0, &"Test")]);
        treestore.insert_with_values(None, None, 
                                     &[(0, &"Two"), (1, &"A")]);
        let beg = treestore.iter_first().unwrap();
        treestore.iter_next(&beg); //Allows us to navigate
        treestore.insert_with_values(Some(&beg), None, &[(0, &"Three")]);
        treestore.insert_with_values(Some(&beg), None, &[(0, &"Four")]); 

        let layout = gtk::Box::new(gtk::Orientation::Vertical, 3);
        let column = gtk::TreeViewColumn::new();
        
        layout.add(&topbox);
        layout.add(&treeview);
        layout.add(&bottombox); 
        treeview.set_headers_visible(false);
        column.set_clickable(true);

        let c2 = gtk::TreeViewColumn::new();
        c2.set_clickable(true);
        let renderer = gtk::CellRendererText::new();
        column.pack_end(&renderer, true);
        column.add_attribute(&renderer, "text", 0);

        let rend2 = gtk::CellRendererText::new();
        c2.pack_end(&rend2, true);
        c2.add_attribute(&rend2, "text", 1);
        column.set_expand(true);
        c2.set_expand(true);

        rend2.set_xalign(1.0);
        treeview.set_expand(true);
        treeview.append_column(&column);
        treeview.append_column(&c2);
                
        window.add(&layout);
        window.show_all();  

    });

    application.run();
}

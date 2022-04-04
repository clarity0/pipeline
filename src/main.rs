use adw::glib::clone;
use adw::prelude::*;
use adw::glib;
use adw::{ActionRow, ApplicationWindow, HeaderBar};
use adw::gtk::{Application, Box, ListBox, Orientation, Entry};

use ytdl::YoutubeDl;

fn main() {
    let application = Application::builder()
        .application_id("com.example.FirstAdwaitaApp")
        .build();

    application.connect_startup(|_| {
        adw::init();
    });

    application.connect_activate(|app| {
        let btn_download = ActionRow::builder()
            .activatable(true)
            .selectable(false)
            .title("Download")
            .build();

        let ent_dl_link = Entry::builder()
            .tooltip_text("Paste a youtube download link here")
            .build();

        btn_download.connect_activated(clone!(@weak ent_dl_link => move |_| {
            let dl_link = ent_dl_link.text();
            eprintln!("link: {}", dl_link);
            let _ = YoutubeDl::new(dl_link)
                .youtube_dl_path("yt-dlp")
                .download(true)
                .run()
                .unwrap();
        }));

        let list = ListBox::builder()
            .margin_top(32)
            .margin_end(32)
            .margin_bottom(32)
            .margin_start(32)
            // the content class makes the list look nicer
            .css_classes(vec![String::from("content")])
            .build();
        list.append(&ent_dl_link);
        list.append(&btn_download);

        // Combine the content in a box
        let content = Box::new(Orientation::Vertical, 0);
        // Adwaitas' ApplicationWindow does not include a HeaderBar
        content.append(
            &HeaderBar::builder()
                .title_widget(&adw::WindowTitle::new("First App", ""))
                .build(),
        );
        content.append(&list);

        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(350)
            // add content to window
            .content(&content)
            .build();
        window.show();
    });

    application.run();
}
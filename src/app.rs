use adw::glib;
use adw::glib::clone;
use adw::gtk::{Box, Entry, HeaderBar, ListBox, Orientation};
use adw::Application;
use adw::{prelude::*, ActionRow, ApplicationWindow};

use crate::download::download_link;

pub fn run_app() -> i32 {
	let application = build_app();
	application.run()
}

fn build_app() -> Application {
	let application = Application::builder()
		.application_id("com.example.FirstAdwaitaApp")
		.build();

	application.connect_startup(|_| {
		adw::init();
	});
	application.connect_activate(build_ui);
	application
}

fn build_ui(app: &Application) {
	let content = build_content();
	let box_download = build_download_box();
	let btn_download = build_download_button();
	let ent_download = build_download_entry_box();

	btn_download.connect_activated(clone!(@weak ent_download => move |_| {
		download_link(ent_download.text());
	}));

	box_download.append(&ent_download);
	box_download.append(&btn_download);

	content.append(&box_download);

	let window = build_window(app, &content);
	window.show();
}

fn build_window(app: &Application, content: &Box) -> ApplicationWindow {
	ApplicationWindow::builder()
		.application(app)
		.default_width(350)
		// add content to window
		.content(content)
		.build()
}

fn build_content() -> Box {
	let content = Box::new(Orientation::Vertical, 0);
	content.append(&build_headerbar());
	content
}

fn build_download_box() -> ListBox {
	ListBox::builder()
		.margin_top(32)
		.margin_end(32)
		.margin_bottom(32)
		.margin_start(32)
		// the content class makes the list look nicer
		.css_classes(vec![String::from("content")])
		.build()
}

fn build_download_button() -> ActionRow {
	ActionRow::builder()
		.activatable(true)
		.selectable(false)
		.title("Download")
		.build()
}

fn build_download_entry_box() -> Entry {
	Entry::builder()
		.tooltip_text("Paste a youtube download link here")
		.build()
}

fn build_headerbar() -> HeaderBar {
	HeaderBar::builder()
			.title_widget(&adw::WindowTitle::new("Youtube Download", ""))
			.build()
}
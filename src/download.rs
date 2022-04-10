use ytdl::*;

pub fn download_link(dl_link: impl AsRef<str>) -> YoutubeDlOutput {
	eprintln!("link: {}", dl_link.as_ref());
	YoutubeDl::new(dl_link.as_ref())
		.youtube_dl_path("yt-dlp")
		.download(true)
		.run()
		.unwrap()
}

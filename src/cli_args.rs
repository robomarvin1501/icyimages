use clap::Parser;
use std::path::PathBuf;

/// A simple image viewer CLI, inspired by nsxiv.
#[derive(Default, Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Paths to the image files to display.
    #[arg(value_name = "FILE", required = true)]
    pub files: Vec<PathBuf>,

    /// Start in fullscreen mode (not yet implemented).
    #[arg(short, long)]
    pub fullscreen: bool,

    /// Enable slideshow mode (not yet implemented).
    #[arg(short, long)]
    pub slideshow: bool,

    /// Set the delay for slideshow in seconds (not yet implemented).
    #[arg(long, value_name = "SECONDS", default_value_t = 3)]
    pub delay: u64,

    /// Zoom level (not yet implemented).
    #[arg(short, long, value_name = "LEVEL")]
    pub zoom: Option<f32>,

    /// Rotate the image by degrees (not yet implemented).
    #[arg(short, long, value_name = "DEGREES")]
    pub rotate: Option<i32>,
}

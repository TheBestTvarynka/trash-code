use std::path::PathBuf;

use clap::{command, Args, Parser, Subcommand};
use url::Url;

#[derive(Debug, Clone, Args)]
#[group(required = true, args = ["file", "link"])]
/// Possible types of the file source
struct FileSource {
    /// Path to the image on the device
    #[arg(long)]
    file: Option<PathBuf>,

    /// Url to the image on the Internet
    #[arg(long)]
    link: Option<Url>,
}

#[derive(Debug, Clone, Subcommand)]
enum Command {
    /// Upload image to the Imgur
    Upload {
        /// File to upload
        #[command(flatten)]
        file_source: FileSource,

        /// Folder for the image on the site
        #[arg(long)]
        folder: String,
    },
    // Download image from the Imgur
    Download {
        /// Source image link
        #[arg(long)]
        link: Url,

        /// Path for the image
        #[arg(long)]
        dest_file: PathBuf,
    },
}

/// Img tool config structure
#[derive(Parser, Debug)]
struct Config {
    /// command to execute
    #[command(subcommand)]
    pub command: Command,

    /// Path to the api key file
    #[arg(long, env = "API-KEY")]
    pub api_key: PathBuf,
}

fn main() {
    println!("{:?}", Config::parse());
}

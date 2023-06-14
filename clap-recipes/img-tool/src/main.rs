use std::path::PathBuf;

use clap::{command, Args, Parser, Subcommand};
use url::Url;

#[derive(Debug, Clone, Args)]
#[group(required = true, args = ["file", "link"])]
/// Possible types of the file source
struct FileSource {
    /// Path to the image on the device
    #[arg(long)]
    pub file: Option<PathBuf>,

    /// Url to the image on the Internet
    #[arg(long)]
    pub link: Option<Url>,
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

#[derive(Debug, Clone, Args)]
struct ApiKeyData {
    /// app id
    #[arg(long, requires = "api_app_secret")]
    pub api_app_id: Option<String>,

    /// app secret
    #[arg(long, requires = "api_app_id")]
    pub api_app_secret: Option<String>,
}

#[derive(Debug, Clone, Args)]
#[group(required = true, args = ["api_key_file", "api_app_id", "api_app_secret"])]
/// Possible types of the api key source
struct ApiKey {
    /// Path to the json file with API key
    #[arg(long)]
    api_key_file: Option<PathBuf>,

    /// Specify API key data in args
    #[command(flatten)]
    api_key_data: ApiKeyData,
}

/// Img tool config structure
#[derive(Parser, Debug)]
struct Config {
    /// command to execute
    #[command(subcommand)]
    pub command: Command,

    /// API key data
    #[command(flatten)]
    pub api_key: ApiKey,
}

fn main() {
    println!("{:?}", Config::parse());
}

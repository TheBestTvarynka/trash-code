use clap::{Parser, Subcommand};
use tonic::transport::Channel;

pub mod bookstore {
    tonic::include_proto!("bookstore");
}

use bookstore::{
    bookstore_service_client::BookstoreServiceClient,
    CreateBookRequest, GetBookRequest,
};

#[derive(Parser)]
#[command(name = "bookstore-client", about = "gRPC Bookstore CLI client")]
struct Cli {
    #[arg(long, default_value = "http://[::1]:8080")]
    server: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    GetBook {
        #[arg(long)]
        id: u64,
    },
    CreateBook {
        #[arg(long)]
        title: String,
        #[arg(long)]
        author: String,
        #[arg(long, default_value_t = 0)]
        year: u32,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    println!("Connecting to {} ...", cli.server);
    let channel = Channel::from_shared(cli.server)?.connect().await?;
    let mut client = BookstoreServiceClient::new(channel);
    println!("Connected.");

    match cli.command {
        Commands::GetBook { id } => {
            println!("Sending GetBook request (id={id}) ...");
            let response = client
                .get_book(GetBookRequest { id })
                .await?
                .into_inner();

            match response.book {
                Some(book) => println!(
                    "Book found:\n  id:     {}\n  title:  {}\n  author: {}\n  year:   {}",
                    book.id, book.title, book.author, book.year
                ),
                None => println!("Server returned an empty book — this shouldn't happen."),
            }
        }

        Commands::CreateBook { title, author, year } => {
            println!("Sending CreateBook request (title=\"{title}\", author=\"{author}\", year={year}) ...");
            let response = client
                .create_book(CreateBookRequest { title, author, year })
                .await?
                .into_inner();

            match response.book {
                Some(book) => println!(
                    "Book created successfully:\n  id:     {}\n  title:  {}\n  author: {}\n  year:   {}",
                    book.id, book.title, book.author, book.year
                ),
                None => println!("Server returned an empty response."),
            }
        }
    }

    Ok(())
}

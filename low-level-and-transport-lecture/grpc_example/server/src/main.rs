use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use tokio::sync::RwLock;
use tonic::{transport::Server, Request, Response, Status};

pub mod bookstore {
    tonic::include_proto!("bookstore");
}

use bookstore::{
    bookstore_service_server::{BookstoreService, BookstoreServiceServer},
    Book, CreateBookRequest, CreateBookResponse, GetBookRequest, GetBookResponse,
};

#[derive(Debug, Default)]
struct BookstoreState {
    books: RwLock<HashMap<u64, Book>>,
    next_id: AtomicU64,
}

#[derive(Debug, Default)]
struct BookstoreServiceImpl {
    state: Arc<BookstoreState>,
}

#[tonic::async_trait]
impl BookstoreService for BookstoreServiceImpl {
    async fn get_book(
        &self,
        request: Request<GetBookRequest>,
    ) -> Result<Response<GetBookResponse>, Status> {
        let id = request.into_inner().id;
        let books = self.state.books.read().await;

        match books.get(&id) {
            Some(book) => Ok(Response::new(GetBookResponse {
                book: Some(book.clone()),
            })),
            None => Err(Status::not_found(format!("Book with id={id} not found"))),
        }
    }

    async fn create_book(
        &self,
        request: Request<CreateBookRequest>,
    ) -> Result<Response<CreateBookResponse>, Status> {
        let req = request.into_inner();

        if req.title.is_empty() {
            return Err(Status::invalid_argument("title must not be empty"));
        }
        if req.author.is_empty() {
            return Err(Status::invalid_argument("author must not be empty"));
        }

        let id = self.state.next_id.fetch_add(1, Ordering::Relaxed) + 1;
        let book = Book {
            id,
            title: req.title,
            author: req.author,
            year: req.year,
        };

        self.state.books.write().await.insert(id, book.clone());

        println!("Created book: id={id}, title=\"{}\"", book.title);

        Ok(Response::new(CreateBookResponse { book: Some(book) }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:8080".parse()?;
    println!("BookstoreService listening on {addr}");

    let svc = BookstoreServiceImpl::default();

    Server::builder()
        .add_service(BookstoreServiceServer::new(svc))
        .serve(addr)
        .await?;

    Ok(())
}

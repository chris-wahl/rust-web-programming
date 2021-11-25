#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{App, dev::Service, HttpResponse, HttpServer};
use futures::future::{Either, ok};

mod auth;
mod database;
mod json_serialization;
mod models;
mod processes;
mod schema;
mod state;
mod to_do;
mod views;

const ADDR: &str = "127.0.0.1:8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!["{} - {}", req.method(), req.path()];

                let passed = if *&req.path().contains("/item/") {
                    match auth::process_token(&req) {
                        Ok(_token) => true,
                        Err(message) => false,
                    }
                } else {
                    true
                };
                // Using the futures::Either enum instead of separate
                // `async {}` blocks, as each `async {}` block is treated like a closure; they
                // each have their own types.  Rust will dislike if one block is returned on `passed` true,
                // and a different one on `passed` false, as they'd effectively be different types.
                //
                //  The Either enum wraps the futures into effectively the same type.
                let end_result = match passed {
                    // token passed, or is not required
                    true => Either::Left(srv.call(req)),
                    // token failed
                    false => {
                        Either::Right(
                            ok( // ok() type makes a future that is immediately ready to go
                                req.into_response(
                                    HttpResponse::Unauthorized().finish().into_body()
                                )
                            )
                        )
                    }
                };
                end_result
            })
            .configure(views::views_factory);

        println!["http://{}", ADDR];
        return app;
    })
        .bind(ADDR)?
        .run()
        .await
}

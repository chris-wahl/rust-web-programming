use actix_web::{App, HttpServer};

mod views;

const ADDR: &str = "127.0.0.1:8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let app = App::new().configure(views::views_factory);

        println!["Started on http://{}", ADDR];
        return app;
    })
        .bind(ADDR)?
        .run()
        .await
}

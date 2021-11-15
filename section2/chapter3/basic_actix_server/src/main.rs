use actix_web::{App, HttpRequest, HttpServer, Responder, web};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

const IP: &str = "127.0.0.1";
const PORT: &str = "8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = format!("{}:{}", IP, PORT);
    println!("Launching server at http://{}", addr);
    HttpServer::new(|| {
        println!["function is firing"];
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    }) // omitting the .workers(int) call here will set the number of workers to the number of CPU cores
        .bind(addr)?
        .run()
        .await
}

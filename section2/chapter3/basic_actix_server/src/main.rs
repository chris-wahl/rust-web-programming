use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use futures::future::try_join3;

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

async fn utils_one() -> impl Responder {
    "Utils one reached\n"
}

async fn health() -> impl Responder {
    "All good\n"
}

const IP: &str = "127.0.0.1";
const PORT: &str = "8000";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let addr = format!("{}:{}", IP, PORT);
    println!("Launching server at http://{}", addr);

    let s1 = HttpServer::new(move || {
        App::new().service(web::scope("/utils").route(
            "/one", web::get().to(utils_one)))
    })
        .bind("0.0.0.0:3006")?
        .run();
    let s2 = HttpServer::new(move || {
        App::new().service(web::resource(
            "/health").route(web::get().to(health)))
    })
        .bind("0.0.0.0:8080")?
        .run();

    let s3 = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    }) // omitting the .workers(int) call here will set the number of workers to the number of CPU cores
        .bind(addr)?
        .run();

    try_join3(s1, s2, s3).await?;

    Ok(())
}

use gomarket_items::create_app::create_app;
use actix_web::HttpServer;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    println!("Starting server on port 8000");
    let server = HttpServer::new(move || create_app()).bind(("0.0.0.0", 8000))?;
    server.run().await
}

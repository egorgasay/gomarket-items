use gomarket_items::create_app::create_app;
use actix_web::HttpServer;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(move || create_app()).bind(("127.0.0.1", 8000))?;
    server.run().await
}

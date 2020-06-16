use actix_web::{get, web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", info.1, info.0)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;

    println!("listening on {} ...", port);

    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:".to_string() + &port.to_string())?
        .run()
        .await
}

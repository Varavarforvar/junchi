use actix_web::{web, App, HttpServer, HttpResponse, Responder, get };
use listenfd::ListenFd;

extern crate diesel;

// use server::establish_connection;

async fn index() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  let mut listenfd = ListenFd::from_env();
  // let connection = establish_connection();

  let mut server = HttpServer::new(|| App::new().route("/", web::get().to(index)));

  server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
    server.listen(l)?
  } else {
    server.bind("127.0.0.1:3000")?
  };

  server.run().await
}

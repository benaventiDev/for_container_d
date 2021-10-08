use actix_web::{App, HttpServer, web, Responder, HttpResponse};
mod config;
mod create;
use dotenv::dotenv;


static COUNTER: i32 = 0;
static SQL_TIME: i32 = 0;
static COSMOS_TIME: i32 = 0;


pub fn views_factory(app: &mut web::ServiceConfig) {
    // define the path struct
    app.route("/endpoint/rust", web::post().to(create::create));
    app.route("/publicar", web::post().to(create::create));
    app.route("/iniciarCarga", web::get().to(create::startLoad));
    app.route("/finalizarCarga", web::get().to(create::endLoad));
}


/*
async fn status() -> impl Responder{
    //web::HttpResponse::Ok().json(Status{status: "Ok".to_string()})
}*/


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();
    HttpServer::new(|| {
        let app = App::new().configure(views_factory);
        return app
    })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
     
}

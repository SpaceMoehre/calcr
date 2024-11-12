use ntex::web::{self, error, middleware, App, Error, HttpResponse};
use tera::Tera;

#[web::get("/")]
async fn serve_index(
    tmpl: web::types::State<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let s = tmpl.render("index.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}


#[web::get("/random_convert")]
pub async fn serve_random_convert(req_body: String) -> impl web::Responder {
    web::HttpResponse::Ok().body(req_body)
}

#[web::get("/convert.html")]
async fn serve_convert(
    tmpl: web::types::State<tera::Tera>,
) -> Result<HttpResponse, Error> {
    let s = tmpl.render("convert.html", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[ntex::main]
pub async fn start_server() -> std::io::Result<()> {
    web::server(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .state(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(serve_index)
            .service(serve_random_convert)
            .service(serve_convert) 
    })
    .workers(4)
    .bind(("0.0.0.0", 28080))?
    .run()
    .await
}
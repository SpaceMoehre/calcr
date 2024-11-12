use ntex::web::{self, error::{self, HttpError}, middleware, App, Error, HttpResponse};
use tera::Tera;
use super::generator;

use rand::Rng;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GenerateRequest{ 
    from: u8, 
    to: u8
}
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


// Define the generate endpoint to handle the JSON POST request.
// #[web::post("/generate")]
async fn api_generate(req: web::types::Json<GenerateRequest>) -> Result<web::HttpResponse, HttpError> {
    let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
    let obj: generator::Task = generator::generate_convert(rng.gen_range(0..4096), req.from, req.to);
    Ok(HttpResponse::Ok().content_type("application/json").json(&obj))
}

#[ntex::main]
pub async fn start_server(args: super::Args) -> std::io::Result<()> {
    web::server(|| {
        let tera =
            Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .state(tera)
            .wrap(middleware::Logger::default()) // enable logger
            .service(serve_index)
            .service(serve_random_convert)
            .service(serve_convert) 
            .service(web::resource("/api/generate").route(web::post().to(api_generate)))

    })
    .workers(4)
    .bind((args.bind_address.as_str(), args.port))?
    .run()
    .await
}

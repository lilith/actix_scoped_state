use std::io;
use std::sync::atomic::{AtomicUsize, Ordering};
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};

#[derive(Debug)]
struct Counter(AtomicUsize);

async fn scope_a(
    counter_a: web::Data<Counter>,
    _req: HttpRequest,
) -> HttpResponse {
    counter_a.0.fetch_add(1, Ordering::SeqCst);

    let body = format!(
        "counter a (increment by visiting /a/): {}",
        counter_a.0.load(Ordering::SeqCst),
    );
    HttpResponse::Ok().body(body)
}

async fn scope_b(
    counter_b: web::Data<Counter>,
    _req: HttpRequest,
) -> HttpResponse {
    counter_b.0.fetch_add(1, Ordering::SeqCst);

    let body = format!(
        "counter b (increment by visiting /b/): {}",
        counter_b.0.load(Ordering::SeqCst),
    );
    HttpResponse::Ok().body(body)
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    // Create some global state prior to building the server

    // move is necessary to give closure below ownership of counter1
    HttpServer::new(move || {
        let counter_a = web::Data::new(Counter(AtomicUsize::new(0usize)));
        let counter_b = web::Data::new(Counter(AtomicUsize::new(0usize)));

        App::new()
            .service(
            web::scope("/a")
                .data(counter_a)
                .default_service(web::to(scope_a)))
            .service(
                web::scope("/b")
                .data(counter_b)
                .default_service(web::to(scope_b)))

            // enable logger
            .wrap(middleware::Logger::default())
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
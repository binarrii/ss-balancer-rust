#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use actix_web::{App, HttpResponse, HttpServer, middleware, web};

use crate::core::estimator::Estimator;
use crate::core::ProxyServer;

mod core;

lazy_static! {
    static ref PROXIES: Vec<ProxyServer> = vec![
        ProxyServer {
            name: "hongk.binarii.me",
            host: "127.0.0.1",
            port: 40001,
            ..Default::default()
        },
        ProxyServer {
            name: "tokyo.binarii.me",
            host: "127.0.0.1",
            port: 40002,
            ..Default::default()
        },
    ];
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=warn");
    env_logger::init();

    for p in PROXIES.iter() {
        Estimator { proxy_server: p }.start()
    }

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| {
                HttpResponse::Ok().json(select())
            }))
    })
    .bind("127.0.0.1:50001")?
    .run()
    .await
}

fn select() -> Vec<&'static ProxyServer> {
    let min = PROXIES.iter()
        .map(|x| x.get_latency())
        .min()
        .unwrap_or(0);

    let selection = PROXIES.iter()
        .filter(|x| x.get_latency() - min <= 200)
        .collect::<Vec<_>>();

    println!("{}", serde_json::to_string(&selection).unwrap());

    selection
}

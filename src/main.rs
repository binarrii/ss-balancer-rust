#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use std::sync::atomic::{AtomicU64, Ordering};

use actix_web::{App, HttpResponse, HttpServer, middleware, web};

use crate::core::config::Config;
use crate::core::estimator::Estimator;
use crate::core::ProxyServer;

mod core;

lazy_static! {
    static ref COUNTER: AtomicU64 = AtomicU64::new(1_0000_0001);
    static ref CONFIG: Config = {
        let data = std::fs::read_to_string("config.json")
            .expect("Config file not found");
        let conf: Config = serde_json::from_str(data.as_str())
            .expect("Can not parse config file");
        conf
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=warn");
    env_logger::init();

    for p in CONFIG.proxies.iter() {
        Estimator { proxy_server: p }.start()
    }

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(|| {
                HttpResponse::Ok().json(select())
            }))
    })
    .bind(format!("{}:{}", CONFIG.address, CONFIG.port))?
    .run()
    .await
}

fn select() -> Vec<&'static ProxyServer> {
    let min = CONFIG.proxies.iter()
        .map(|x| x.get_latency())
        .min()
        .unwrap_or(0);

    let selection = CONFIG.proxies.iter()
        .filter(|x| x.get_latency() - min <= CONFIG.tolerance.unwrap_or(200))
        .collect::<Vec<_>>();

    let i = COUNTER.fetch_add(1, Ordering::SeqCst);
    let s = serde_json::to_string(&selection).unwrap();

    println!("{} >> {}", i, s);

    selection
}

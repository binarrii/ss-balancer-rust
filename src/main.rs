#[macro_use]
extern crate lazy_static;
extern crate serde_json;

use std::sync::atomic::{AtomicU64, Ordering};

use actix_web::{App, HttpResponse, HttpServer, middleware, web};
use rand::Rng;

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
        .map(|x| x.get_latency()).min().unwrap_or(0);

    let tolerance = CONFIG.tolerance.unwrap_or(200);
    let selection = CONFIG.proxies.iter()
        .filter(|x| x.get_latency() - min <= tolerance)
        .filter(|x| x.weight > 0)
        .collect::<Vec<_>>();

    if selection.is_empty() { return selection; }

    let mut partitions = vec![0];
    for (pos, x) in selection.iter().enumerate() {
        partitions.push(partitions[pos] + x.weight)
    }

    let weight_total = selection.iter().map(|x| x.weight).sum();
    let rand = rand::thread_rng().gen_range(0..weight_total);

    let mut result = selection[0];
    for x in 1..partitions.len() {
        if rand < partitions[x] {
            result = selection[x - 1];
            break;
        }
    }

    let i = COUNTER.fetch_add(1, Ordering::SeqCst);
    let s = serde_json::to_string(result).unwrap();

    println!("{} >> {}", i, s);

    vec![result]
}

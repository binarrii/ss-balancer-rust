use std::convert::TryFrom;
use std::ops::Deref;
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;

use crate::core::ProxyServer;

const TEST_URIS: [&str; 1] = [
    // "https://www.google.com",
    "https://www.baidu.com",
    // "https://www.twitter.com",
    // "https://www.instagram.com"
];

const ROUNDS: usize = 5;

#[derive(Clone)]
pub struct Estimator<'a> {
    pub proxy_server: &'a ProxyServer
}

impl<'a> Estimator<'a>
    where 'a: 'static
{
    pub fn start(self) {
        thread::spawn(move || loop { self.clone().estimate(); });
    }

    fn estimate(self) {
        let proxy = reqwest::Proxy::http(&self.proxy_server.format())
            .expect("Invalid proxy server");

        let client = reqwest::blocking::Client::builder()
            .proxy(proxy.clone())
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Can't build a http client");

        let mut total: u128 = 0;
        for _ in 1..ROUNDS {
            let secs = rand::thread_rng().gen_range(10..30);
            thread::sleep(Duration::from_secs(secs));

            for uri in TEST_URIS.iter() {
                let now = Instant::now();
                let result = client.head(uri.deref()).send();
                let elapsed = match result {
                    Ok(_) => now.elapsed().as_millis(),
                    Err(_) => 10000
                };
                total = total + elapsed;
            }
        }
        let x = u128::try_from(ROUNDS * TEST_URIS.len()).unwrap();
        let y = total / x;

        let rating_guard = self.proxy_server.rating.lock().unwrap();
        rating_guard.set((rating_guard.get() * 3 + y * 7) / 10)
    }
}

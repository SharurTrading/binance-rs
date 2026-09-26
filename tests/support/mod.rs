// SPDX-FileCopyrightText: 2026 Kevin Monaghan
// SPDX-License-Identifier: MIT-0

use binance_client::usdm::{Config, Environment};
use binance_client::{Clock, Credentials, Error};
use std::{
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
    sync::mpsc,
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

pub struct FixedClock(pub u64);
impl Clock for FixedClock {
    fn now_millis(&self) -> Result<u64, Error> {
        Ok(self.0)
    }
}
pub fn config() -> Config {
    Config::new(Environment::Demo)
        .unwrap()
        .clock(Arc::new(FixedClock(1_700_000_001_000)))
        .credentials(Credentials::hmac("synthetic-api-key", "synthetic-secret").unwrap())
}
pub fn deadline() -> tokio::time::Instant {
    tokio::time::Instant::now() + Duration::from_secs(3)
}

pub struct HttpFixture {
    pub url: String,
    pub requests: mpsc::UnboundedReceiver<String>,
    count: Arc<AtomicUsize>,
    stop: CancellationToken,
    task: JoinHandle<()>,
}
impl HttpFixture {
    pub async fn new(
        status: u16,
        headers: &str,
        body: &str,
        length: Option<usize>,
        hold: bool,
    ) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let bytes=format!("HTTP/1.1 {status} fixture\r\nConnection: close\r\nContent-Length: {}\r\n{headers}\r\n{body}",length.unwrap_or(body.len())).into_bytes();
        let count = Arc::new(AtomicUsize::new(0));
        let counter = count.clone();
        let stop = CancellationToken::new();
        let cancel = stop.clone();
        let (tx, requests) = mpsc::unbounded_channel();
        let task = tokio::spawn(async move {
            loop {
                let accepted = tokio::select! {()=cancel.cancelled()=>break,result=listener.accept()=>result.unwrap()};
                let (mut stream, _) = accepted;
                counter.fetch_add(1, Ordering::SeqCst);
                let request = tokio::select! {()=cancel.cancelled()=>break,result=read_request(&mut stream)=>result};
                tx.send(request).unwrap();
                stream.write_all(&bytes).await.unwrap();
                if hold {
                    cancel.cancelled().await;
                    break;
                }
            }
        });
        Self {
            url,
            requests,
            count,
            stop,
            task,
        }
    }
    pub fn attempts(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
    pub async fn finish(self) {
        self.stop.cancel();
        self.task.await.unwrap();
    }
}
async fn read_request(stream: &mut tokio::net::TcpStream) -> String {
    let mut bytes = Vec::new();
    let mut buf = [0; 4096];
    loop {
        let n = stream.read(&mut buf).await.unwrap();
        assert!(n > 0);
        bytes.extend_from_slice(&buf[..n]);
        if let Some(end) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
            let header = String::from_utf8(bytes[..end].to_vec()).unwrap();
            let length = header
                .lines()
                .find_map(|l| {
                    l.to_lowercase()
                        .strip_prefix("content-length: ")
                        .map(str::to_owned)
                })
                .map_or(0, |v| v.parse::<usize>().unwrap());
            if bytes.len() >= end + 4 + length {
                return String::from_utf8(bytes).unwrap();
            }
        }
    }
}

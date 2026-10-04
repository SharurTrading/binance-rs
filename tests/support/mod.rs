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
/// Transport race injected by [`HttpFixture::keep_alive`]. Exists for the
/// HTTP contract binary only, but `support` compiles into every test binary.
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum KeepAliveFault {
    /// The n-th request is read in full, then the connection closes without
    /// responding: the venue may have acted, and the answer is lost.
    LostAcknowledgement(usize),
    /// The connection closes after the n-th response, while it sits pooled
    /// and idle: the next dispatch lands on a connection the client does not
    /// yet know is dead.
    StaleIdle(usize),
}
/// Per-connection serving policy shared by both fixture constructors.
#[derive(Clone, Copy)]
enum Policy {
    /// One request per connection; each response carries `Connection: close`.
    Close { hold: bool },
    /// Keep-alive responses with optional fault injection.
    KeepAlive { fault: Option<KeepAliveFault> },
}
impl HttpFixture {
    pub async fn new(
        status: u16,
        headers: &str,
        body: &str,
        length: Option<usize>,
        hold: bool,
    ) -> Self {
        let bytes = format!(
            "HTTP/1.1 {status} fixture\r\nConnection: close\r\nContent-Length: {}\r\n{headers}\r\n{body}",
            length.unwrap_or(body.len())
        )
        .into_bytes();
        Self::spawn(bytes, Policy::Close { hold }).await
    }
    /// Keep-alive fixture: responses omit `Connection: close`, so the client
    /// pools the connection and later calls reuse it. [`KeepAliveFault`]
    /// injects the transport races the contract tests pin, and the listener
    /// keeps accepting in both cases, so a speculative extra send is
    /// observable as another accepted connection. This constructor exists
    /// for the HTTP contract binary only, but `support` compiles into every
    /// test binary.
    #[allow(dead_code)]
    pub async fn keep_alive(
        status: u16,
        headers: &str,
        body: &str,
        fault: Option<KeepAliveFault>,
    ) -> Self {
        let bytes = format!(
            "HTTP/1.1 {status} fixture\r\nContent-Length: {}\r\n{headers}\r\n{body}",
            body.len()
        )
        .into_bytes();
        Self::spawn(bytes, Policy::KeepAlive { fault }).await
    }
    async fn spawn(bytes: Vec<u8>, policy: Policy) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let count = Arc::new(AtomicUsize::new(0));
        let counter = count.clone();
        let stop = CancellationToken::new();
        let cancel = stop.clone();
        let (tx, requests) = mpsc::unbounded_channel();
        let task = tokio::spawn(async move {
            let mut served = 0;
            loop {
                let accepted = tokio::select! {()=cancel.cancelled()=>break,result=listener.accept()=>result.unwrap()};
                let (mut stream, _) = accepted;
                counter.fetch_add(1, Ordering::SeqCst);
                match policy {
                    Policy::Close { hold } => {
                        let request = tokio::select! {()=cancel.cancelled()=>break,result=read_request(&mut stream)=>result};
                        tx.send(request).unwrap();
                        stream.write_all(&bytes).await.unwrap();
                        if hold {
                            cancel.cancelled().await;
                            break;
                        }
                    }
                    Policy::KeepAlive { fault } => loop {
                        let request = tokio::select! {()=cancel.cancelled()=>break,result=read_request(&mut stream)=>result};
                        tx.send(request).unwrap();
                        served += 1;
                        if matches!(fault, Some(KeepAliveFault::LostAcknowledgement(n)) if n == served)
                        {
                            break;
                        }
                        stream.write_all(&bytes).await.unwrap();
                        if matches!(fault, Some(KeepAliveFault::StaleIdle(n)) if n == served) {
                            break;
                        }
                    },
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
    /// Accepted TCP connections — not requests sent: with keep-alive pooling,
    /// one connection can carry many requests.
    pub fn connections_accepted(&self) -> usize {
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

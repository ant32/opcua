use std::io;
use std::sync::{Arc, RwLock};
use std::thread;

use futures;
use futures::{Poll, Async};
use futures::future::Future;
use hyper;
use hyper::{Method, StatusCode};
use hyper::header::ContentType;
use hyper::server::{Http, NewService, Request, Response, Service};
use serde_json;
use server::Connections;

use metrics::ServerMetrics;
use state::ServerState;

/// This is our metrics service, the thing called to handle requests coming from hyper
struct MetricsService {
    server_state: Arc<RwLock<ServerState>>,
    connections: Arc<RwLock<Connections>>,
    server_metrics: Arc<RwLock<ServerMetrics>>,
}

impl MetricsService {
    fn new(server_state: Arc<RwLock<ServerState>>, connections: Arc<RwLock<Connections>>, server_metrics: Arc<RwLock<ServerMetrics>>) -> MetricsService {
        MetricsService {
            server_state,
            connections,
            server_metrics,
        }
    }
}

impl Service for MetricsService {
    // boilerplate hooking up hyper's server types
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;

    // The future representing the eventual Response the call will resolve to
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();
        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                let content = include_str!("index.html");
                response.set_body(content);
            }
            (&Method::Get, "/metrics") => {
                use std::ops::Deref;
                // Send metrics data as json
                let json = {
                    let mut server_metrics = self.server_metrics.write().unwrap();
                    {
                        let server_state = self.server_state.read().unwrap();
                        server_metrics.update_from_server_state(&server_state);
                    }
                    {
                        let connections = self.connections.read().unwrap();
                        let connections = connections.deref();
                        server_metrics.update_from_connections(connections);
                    }
                    serde_json::to_string_pretty(server_metrics.deref()).unwrap()
                };
                response.headers_mut().set(ContentType::json());
                response.set_body(json);
            }
            _ => {
                response.set_status(StatusCode::NotFound);
            }
        }
        Box::new(futures::future::ok(response))
    }
}

struct MetricsServiceFactory {
    server_state: Arc<RwLock<ServerState>>,
    connections: Arc<RwLock<Connections>>,
    server_metrics: Arc<RwLock<ServerMetrics>>,
}

impl NewService for MetricsServiceFactory {
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Instance = MetricsService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(MetricsService::new(self.server_state.clone(), self.connections.clone(), self.server_metrics.clone()))
    }
}

struct HttpQuit {
    server_state: Arc<RwLock<ServerState>>
}

impl Future for HttpQuit {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let abort = {
            let server_state = trace_read_lock_unwrap!(self.server_state);
            server_state.is_abort()
        };
        if abort {
            Ok(Async::Ready(()))
        } else {
            Ok(Async::NotReady)
        }
    }
}

/// Runs an http server on the specified binding address, serving out the supplied server metrics
pub fn run_http_server(address: &str, server_state: Arc<RwLock<ServerState>>, connections: Arc<RwLock<Connections>>, server_metrics: Arc<RwLock<ServerMetrics>>) -> thread::JoinHandle<()> {
    let address = address.parse().unwrap();
    thread::spawn(move || {
        // This polling action will quit the http server when the OPC UA server aborts
        let server_should_quit = HttpQuit { server_state: server_state.clone() };

        info!("HTTP server is running on {} to provide OPC UA server metrics", address);
        let metrics_factory = MetricsServiceFactory {
            server_state,
            connections,
            server_metrics,
        };
        let http_server = Http::new().bind(&address, metrics_factory).unwrap();
        http_server.run_until(server_should_quit).unwrap();
        info!("HTTP server has stopped");
    })
}

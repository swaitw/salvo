use salvo::prelude::*;

#[handler]
async fn hello() -> &'static str {
    "Hello World"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let mut router = Router::new().get(hello);
    let listener = TcpListener::new("0.0.0.0:443")
        .acme()
        .cache_path("temp/letsencrypt")
        .add_domain("test.salvo.rs") // Replace this domain name with your own.
        .http01_challenge(&mut router)
        .quinn("0.0.0.0:443");
    let acceptor = listener.join(TcpListener::new("0.0.0.0:80")).bind().await;
    Server::new(acceptor).serve(router).await;
}

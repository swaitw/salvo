use salvo::cors::Cors;
use salvo::http::Method;
use salvo::prelude::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    tokio::join!(backend_server(), frontend_server());
}

async fn backend_server() {
    #[handler]
    async fn hello() -> &'static str {
        "hello, I am content from remote server."
    }

    let cors = Cors::new()
        .allow_origin(["http://127.0.0.1:5800", "http://localhost:5800"])
        .allow_methods(vec![Method::GET, Method::POST, Method::DELETE])
        .allow_headers("authorization")
        .into_handler();

    let router = Router::with_path("hello").post(hello);
    let service = Service::new(router).hoop(cors);

    let acceptor = TcpListener::new("0.0.0.0:5600").bind().await;
    Server::new(acceptor).serve(service).await;
}

async fn frontend_server() {
    #[handler]
    async fn index() -> Text<&'static str> {
        Text::Html(HTML_DATA)
    }

    let router = Router::new().get(index);
    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}

const HTML_DATA: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width">
    <title>Salvo Cors</title>
</head>
<body>
<button id="btn">Load Content</button>
<div id="content"></div>
<script>
document.getElementById("btn").addEventListener("click", function() {
    fetch("http://127.0.0.1:5600/hello", {method: "POST", headers: {authorization: "abcdef"}}).then(function(response) {
        return response.text();
    }).then(function(data) {
        document.getElementById("content").innerHTML = data;
    });
});
</script>
</body>
</html>
"#;

use salvo::basic_auth::{BasicAuth, BasicAuthValidator};
use salvo::prelude::*;

struct Validator;
impl BasicAuthValidator for Validator {
    async fn validate(&self, username: &str, password: &str, _depot: &mut Depot) -> bool {
        username == "root" && password == "pwd"
    }
}
#[handler]
async fn hello() -> &'static str {
    "Hello"
}

fn route() -> Router {
    let auth_handler = BasicAuth::new(Validator);
    Router::with_hoop(auth_handler).goal(hello)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let acceptor = TcpListener::new("0.0.0.0:5800").bind().await;
    Server::new(acceptor).serve(route()).await;
}

#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    #[tokio::test]
    async fn test_basic_auth() {
        let service = Service::new(super::route());

        let content = TestClient::get("http://0.0.0.0:5800/")
            .basic_auth("root", Some("pwd"))
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();
        assert!(content.contains("Hello"));

        let content = TestClient::get("http://0.0.0.0:5800/")
            .basic_auth("root", Some("pwd2"))
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();
        assert!(content.contains("Unauthorized"));
    }
}

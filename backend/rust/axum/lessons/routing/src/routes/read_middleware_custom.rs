use axum::Extension;

#[derive(Clone)]
pub struct HeaderMessage(pub String);

pub async fn read_middleware_header(Extension(message): Extension<HeaderMessage>) -> String {
    message.0
}

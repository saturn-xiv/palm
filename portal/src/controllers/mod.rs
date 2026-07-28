use axum::response::Html;

pub async fn home() -> Html<&'static str> {
    // TODO
    "<html><h1>Home</h1></html>".into()
}

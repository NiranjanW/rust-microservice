use axum::{
    routing::get,
    Router
};
// use websocket::{WsServer, WebSocketUpgrade};

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Listening On {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/integration-testable", get(|| async {"Hello world!"}))
        // .route("/unit-testable", get(unit_testable_handler))
}


// async fn integration_testable_handler(ws: WebSocketUpgrade) -> Response {
//     ws.on_upgrade(integration_testable_handle_socket)
// }
//
// async fn integration_testable_handle_socket(mut socket: WebSocket) {
//     while let Some(Ok(msg)) = socket.recv().await {
//         if let Message::Text(msg) = msg {
//             if socket
//                 .send(Message::Text(format!("You said: {msg}")))
//                 .await
//                 .is_err()
//             {
//                 break;
//             }
//         }
//     }

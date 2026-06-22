use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

use crate::{
    daemon::bootstrap::bootstrap, projection::document::generate_runtime_views, state::state,
};

fn init_runtime_state() {
    let mut s = state::load();
    s.starts += 1;
    s.started_at = state::now();
    state::save(&s);
}

async fn serve(listener: TcpListener) {
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let mut buf = [0; 1024];

        let n = socket.read(&mut buf).await.unwrap();

        let cmd = String::from_utf8_lossy(&buf[..n]);

        if cmd.trim() == "status" {
            let s = state::load();
            let out = serde_json::to_string(&s).unwrap();
            socket.write_all(out.as_bytes()).await.unwrap();
        }
        // handle_socket(socket).await;
    }
}

pub async fn start() {
    println!("🏃🏻 leviticus running!");
    let project_root = std::env::current_dir().unwrap();
    bootstrap(project_root.clone());
    init_runtime_state();
    // generate_explain_doc(&workspace);
    generate_runtime_views();
    serve(TcpListener::bind("127.0.0.1:7788").await.unwrap()).await;
}

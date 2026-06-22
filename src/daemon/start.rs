use crate::{
    daemon::{bootstrap::bootstrap, state::DaemonState},
    projection::document::generate_runtime_views,
};

use async_trait::async_trait;
use cli::{CliCommand, Context};

use std::io::{self, Write};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

fn init_runtime_state() {
    let mut s = DaemonState::load();
    s.starts += 1;
    s.started_at = DaemonState::now();
    DaemonState::save(&s);
}

async fn serve(listener: TcpListener) {
    loop {
        let (mut socket, _) = listener.accept().await.unwrap();

        let mut buf = [0; 1024];

        let n = socket.read(&mut buf).await.unwrap();

        let cmd = String::from_utf8_lossy(&buf[..n]);

        if cmd.trim() == "status" {
            let s = DaemonState::load();
            let out = serde_json::to_string(&s).unwrap();
            socket.write_all(out.as_bytes()).await.unwrap();
        }
        // handle_socket(socket).await;
    }
}

pub struct StartDaemon;

#[async_trait::async_trait]
impl CliCommand for StartDaemon {
    async fn run(&self, ctx: &Context) {
        println!("1️⃣ entered start_daemon");
        io::stdout().flush().unwrap();

        let project_root = std::env::current_dir().unwrap();
        println!("2️⃣ got cwd: {:?}", project_root);
        io::stdout().flush().unwrap();

        bootstrap(project_root.clone());
        println!("3️⃣ after bootstrap");
        io::stdout().flush().unwrap();

        init_runtime_state();
        println!("4️⃣ after state");
        io::stdout().flush().unwrap();

        generate_runtime_views();
        println!("5️⃣ after views");
        io::stdout().flush().unwrap();

        println!("6️⃣ about to serve");
        io::stdout().flush().unwrap();

        serve(TcpListener::bind("127.0.0.1:7788").await.unwrap()).await;

        println!("7️⃣ never reached (expected)");
        io::stdout().flush().unwrap();
    }
}

pub async fn start_start() {
    // tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

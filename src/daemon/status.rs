use cli::{CliCommand, Context};

pub struct StatusDaemon;

#[async_trait::async_trait]
impl CliCommand for StatusDaemon {
    async fn run(&self, ctx: &Context) {
        println!("📊 Daemon status: OK");
        let mut stream = tokio::net::TcpStream::connect("127.0.0.1:7788")
            .await
            .unwrap();

        tokio::io::AsyncWriteExt::write_all(&mut stream, b"status")
            .await
            .unwrap();

        let mut buf = vec![];

        tokio::io::AsyncReadExt::read_to_end(&mut stream, &mut buf)
            .await
            .unwrap();

        println!("{}", String::from_utf8_lossy(&buf));
    }
}

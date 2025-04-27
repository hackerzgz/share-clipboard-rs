use clap::Parser;
use std::{
    error::Error,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    sync::Arc,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::RwLock,
};

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
struct Args {
    // enable client-mode if server ip are provided
    #[arg(short, long)]
    server: Option<Ipv4Addr>,

    #[arg(short, long, default_value_t = 7582)]
    port: u16,
}

#[derive(Clone)]
struct Server {
    peers: Arc<RwLock<Vec<SocketAddr>>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            peers: Arc::new(RwLock::new(vec![])),
        }
    }

    pub async fn push_client(&self, client: SocketAddr) {
        let mut peers = self.peers.write().await;
        if !peers.contains(&client) {
            peers.push(client);
        }
    }

    pub async fn drop_client(&self, client: SocketAddr) {
        let mut peers = self.peers.write().await;
        if let Some(pos) = peers.iter().position(|p| *p == client) {
            peers.remove(pos);
        }
    }

    pub async fn broadcast(&self, sender: SocketAddr, data: &[u8]) {
        let peers = self.peers.read().await;
        for &peer in peers.iter() {
            if peer != sender {
                if let Ok(mut stream) = TcpStream::connect(peer).await {
                    let _ = stream.write_all(data).await;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if let Some(server_ip) = args.server {
        // TODO: Client mode (需要实现客户端逻辑)
        println!("Connecting to server: {}", server_ip);
    } else {
        // Server mode
        let addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), args.port);
        let listener = TcpListener::bind(addr).await?;
        println!("server listening on {}", addr);

        let server = Server::new();

        loop {
            let (mut stream, client_addr) = listener.accept().await?;
            let server_clone = server.clone();

            tokio::spawn(async move {
                println!("new connection: {}", client_addr);

                // 添加客户端到列表
                server_clone.push_client(client_addr).await;

                let mut buf = [0; 1024];

                loop {
                    match stream.read(&mut buf).await {
                        Ok(0) => break, // 连接关闭
                        Ok(n) => {
                            // 广播消息给其他客户端
                            server_clone.broadcast(client_addr, &buf[0..n]).await;
                        }
                        Err(e) => {
                            eprintln!("read unexpected error: {}", e);
                            break;
                        }
                    }
                }

                // 移除客户端
                server_clone.drop_client(client_addr).await;
                println!("connection closed: {}", client_addr);
            });
        }
    }

    Ok(())
}

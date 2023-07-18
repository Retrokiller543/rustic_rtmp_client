use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use rand::Rng;

#[tokio::main]
async fn main() {
    let mut c0_c1_buffer = [0; 1537]; // Size for C0 + C1
    let mut s0_s1_s2_buffer = [0; 3073]; // Size for S0 + S1 + S2
    let mut c2_buffer = [0; 1536]; // Size for C2

    // Construct C0.
    c0_c1_buffer[0] = 3; // RTMP version

    // Construct C1.
    let client_timestamp = 0u32.to_be_bytes(); // Client uptime in milliseconds
    c0_c1_buffer[1..5].copy_from_slice(&client_timestamp);
    // Fill the rest of C1 with random data.
    let mut rng = rand::thread_rng();
    rng.fill(&mut c0_c1_buffer[5..1537]);

    // Connect to the server.
    let mut stream = TcpStream::connect("127.0.0.1:1935").await.expect("Could not connect");

    // Send C0 and C1 to the server.
    stream.write_all(&c0_c1_buffer).await.expect("Failed to write to socket");

    // Read S0, S1, and S2 from the server.
    stream.read_exact(&mut s0_s1_s2_buffer).await.expect("Failed to read from socket");
    // Construct C2 by copying S1.
    let s1 = &s0_s1_s2_buffer[1..1537];
    c2_buffer.copy_from_slice(s1);
    println!("S1: {:?}", &s1[0..10]);
    println!("C2: {:?}", &c2_buffer[0..10]);

    // Send C2 to the server.
    stream.write_all(&c2_buffer).await.expect("Failed to write to socket");

    println!("Handshake complete");
}

use server::Server;

mod http;
mod server;

fn main() {
    let address = "127.0.0.1:8080";
    let server = Server::new(address.to_string());

    server.run();
}

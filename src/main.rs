use std::net::TcpListener;
use std::io::ErrorKind;

mod server;
use server::thread::ThreadPool;
use server::process::CliArgs;
use server::request::handle_connection;

fn main() {
    let args = CliArgs::get();
    let server_port: &str;
    match &args.port {
        Some(port) => {
            server_port = port;
        }
        None => {server_port = "0";}
    }
    let server_addr = String::from("127.0.0.1:") + server_port;
    let listener = TcpListener::bind(server_addr);

    let pool = ThreadPool::new(args.thread_count);

    match listener {
        Ok(listener) => {
            println!("App listening on port {}", TcpListener::local_addr(&listener).unwrap());
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                pool.execute(|| {
                    handle_connection(stream);
                });
            }
        }
        Err(e) => {
            match e.kind() {
                ErrorKind::InvalidInput => {
                    panic!("{}", e)
                }
                ErrorKind::AddrInUse => {
                    panic!("{}", e)
                }
                _ => {
                    panic!("Unable to establish connection")
                }
            }
        }
    }
}

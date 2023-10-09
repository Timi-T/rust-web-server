//! # use server::{ [request], [thread], [process] }
//! 
//! Library crate for the rust-web-server

pub mod request;
pub mod thread;
pub mod process;

pub use request::RequestObject as r_RequestObject;
pub use request::handle_connection as r_handle_connection;
pub use thread::ThreadPool as t_ThreadPool;
pub use thread::Worker as t_Worker;
pub use process::CliArgs as p_CliArgs;
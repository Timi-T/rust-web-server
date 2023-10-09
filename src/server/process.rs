//! # use server::process::{ [CliArgs] }
//! 
//! A library that hanldes process related concepts for the web-server.

use std::env;

const ALLOWED_ARGS: [&str; 3] = ["--port", "--threads", "--dir-path"];

/// The options to start the web-server are collected as command line arguments and parsed into a
/// convinient data structure which is used throughout the program.
/// The only supported arguments are `--port`, `--thread`, and `--dir-path`.
pub struct CliArgs {
    /// This represents the `--port` option which binds the application to the specified port.
    /// When not provided, a random port is automatically assigned and displayed on the command line.
    pub port: Option<String>,

    /// This represents the `--thread` option which specifies the number of threads to be used for the server.
    /// When not provided, the program defaults to a thread count of 10.
    pub thread_count: usize,

    /// This represents the `--dir-path` option which specifies the root directory of the static files to be served.
    /// It could be relative to the current directory path or an absolute path.
    /// When not provided, the current directory acts as the root directory for the static files.
    pub dir_path: String
}

impl CliArgs {

    /// Returns a struct with options provided to run the server in the command line.
    /// 
    /// # Allowed command line arguments
    /// 
    /// * `--port` - Specifies the port to bind the application to.
    /// * `--threads` - Specifies the number of threads to serve concurrent requests.
    /// * `--dir-path` - Specifies the path where the static files are located.
    /// 
    /// # Example
    /// 
    /// ```
    /// use server::process::CliArgs;
    /// use std::env;
    /// 
    /// // Default port
    /// let args: CliArgs = CliArgs::get();
    /// assert_eq!(args.thread_count, 10);
    /// 
    /// // Default file path
    /// let env_args: Vec<String> = env::args().collect();
    /// assert_eq!(args.dir_path, env_args.get(0).unwrap().as_str());
    /// 
    /// // No port
    /// assert_eq!(args.port, None);
    /// ```
    /// 
    /// # Panics
    /// 
    /// - Panics if a command line argument is not allowed. Allowed args include `--threads`, `--port`, `--dir-path`.
    /// - Panics if a value is not provided for an argument.
    /// - Panics if the value for `--threads` is not an integer.
    /// 
    pub fn get() -> CliArgs {
        let args: Vec<String> = env::args().collect();
        let mut return_args = CliArgs { port: None, thread_count: 10, dir_path: String::from(args[0].as_str()) };

        let mut i = 1;

        while i < args.len() {
            let arg = &args[i];
            assert!(
                ALLOWED_ARGS.contains(&arg.as_str()),
                "Invalid argument {}", &arg.as_str()
            );

            assert!(i + 1 < args.len(), "Missing value for {}", arg);
            
            if arg == "--port" {
                return_args.port = Some(String::from(args[i + 1].as_str()));
            } else if arg == "--threads" {
                let pool_size = args[i + 1].as_str().parse();
                match pool_size {
                    Ok(size) => {return_args.thread_count = size}
                    Err(_) => {
                        panic!("Unrecognized thread pool size. Please pass in a number")
                    }
                }
            } else if arg == "--dir-path" {
                return_args.dir_path =  String::from(args[i + 1].as_str());
            }
            
            i += 2;
        }

        return_args
    }
}

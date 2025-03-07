
pub mod worker_task;
pub mod worker;
pub mod server_manager;
pub mod request_manager;

use std::sync::atomic::AtomicBool;
use std::sync::Arc;


use server_manager::ServerManager;




fn main() {

    // define base threads count for the server for basic operation
    let base_threads_count : usize = 5;

    // set up the ip address for the server
    let ip_address : &'static str = "localhost";

    // set up the port for the server
    let port : u16 = 8080;

    let is_running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    // Run Manual tests for the Server 
    let mut server_manager = ServerManager::new( base_threads_count,  ip_address, port , is_running);


    // Start the Server
    server_manager.start_server();
    

    // Stop the Server and Drop trait called to clean up
    drop(server_manager);
    
}

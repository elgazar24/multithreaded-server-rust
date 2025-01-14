

pub mod server_manager;
pub mod worker;
pub mod worker_task;


use server_manager::ServerManager;



fn main() {

    // define base threads count for the server for basic operation
    let base_threads_count : usize = 100;

    // define max threads count for the server for safety reason ( Denial of Service "DoS" attack )
    let max_threads_count : usize = 120;

    // set up the ip address for the server
    let ip_address : &'static str = "localhost";

    // set up the port for the server
    let port : u16 = 8080;

    // Run Manual tests for the Server 
    let mut server_manager = ServerManager::new( base_threads_count, max_threads_count, ip_address, port );

    // Start the Server
    server_manager.start_server();

    // Wait for the workers to finish
    server_manager.ensure_tasks_finished();


    
}

use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use multithread_server_task::server_manager::ServerManager;

mod client;
use client::Client;




#[test]
#[ignore = "Performance test is should give you an idea of the performance of your server uncomment to run it and remove the ignore (AFTER FINISH IT FAILS TO GIVE YOU RESULTS)"]
fn performance_test() {

    let client_count: i32 = 4000; 

    let failed_requests: Arc<Mutex<u64>> = Arc::new(Mutex::new(0));
    let request_durations: Arc<Mutex<Vec<Duration>>> = Arc::new(Mutex::new(Vec::new()));

    let ip_address: &str = "localhost";
    let port: u16 = 8080;
    let base_threads_count: usize = 100; 

    let is_running: Arc<AtomicBool> = Arc::new(AtomicBool::new(true));

    let is_running_clone: Arc<AtomicBool> = Arc::clone(&is_running);

    // Start the server
    let mut server_manager: ServerManager = ServerManager::new(base_threads_count, ip_address, port , is_running);

    // Run server in a separate thread
    thread::spawn(move || {

        server_manager.start_server();

        thread::sleep(Duration::from_secs(1));
    });

    // Wait for the server to initialize
    thread::sleep(Duration::from_secs(1)); 

    let start_time: Instant = Instant::now();
    let mut handles: Vec<thread::JoinHandle<()>> = vec![];

    for _ in 0..client_count {
        let failed_requests_clone: Arc<Mutex<u64>> = Arc::clone(&failed_requests);
        let request_durations_clone: Arc<Mutex<Vec<Duration>>> = Arc::clone(&request_durations);

        let handle: thread::JoinHandle<()> = thread::spawn(move || {
            let start: Instant = Instant::now();
            let client: Client = Client::new("localhost", 8080);

            let mut stream: std::net::TcpStream = match client.connect() {
                Ok(s) => s,
                Err(_) => {
                    let mut failed: std::sync::MutexGuard<'_, u64> = failed_requests_clone.lock().unwrap();
                    *failed += 1;
                    return;
                }
            };

            let request: &str = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            if client.send(request, &mut stream).is_err() {
                let mut failed = failed_requests_clone.lock().unwrap();
                *failed += 1;
                return;
            }

            let res = client.receive(&mut stream);

            match res {
                Ok(response) => {
                    if !response.contains("HTTP/1.1 200 OK") {
                        let mut failed: std::sync::MutexGuard<'_, u64> = failed_requests_clone.lock().unwrap();
                        *failed += 1;
                        return;
                    }
                }
                Err(error) => {
                    println!("Error: {}", error);
                    let mut failed: std::sync::MutexGuard<'_, u64> = failed_requests_clone.lock().unwrap();
                    *failed += 1;
                    return;
                }
                
            }

            let duration = start.elapsed();
            let mut durations = request_durations_clone.lock().unwrap();
            durations.push(duration);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let total_duration = start_time.elapsed();
    let total_requests = client_count as u64;
    let successful_requests = total_requests - *failed_requests.lock().unwrap();

    let avg_response_time: f64 = {
        let durations = request_durations.lock().unwrap();
        durations.iter().map(|&d| d.as_secs_f64()).sum::<f64>() / successful_requests as f64
    };

    is_running_clone.store(false, std::sync::atomic::Ordering::SeqCst);

    let mut message = "Performance Test Finished".to_string();
    message += &format!("\nTotal Requests: {}", total_requests);
    message += &format!("\nSuccessful Requests: {}", successful_requests);
    message += &format!("\nFailed Requests: {}", *failed_requests.lock().unwrap());
    message += &format!(
        "\nPercentage of Successful Requests: {:.2}%",
        successful_requests as f64 / total_requests as f64 * 100.0
    );
    message += &format!("\nTotal Duration: {:.2} seconds", total_duration.as_secs_f64());
    message += &format!(
        "\nRequests per Second: {:.2}",
        successful_requests as f64 / total_duration.as_secs_f64()
    );
    message += &format!(
        "\nAverage Response Time (seconds): {:.4}",
        avg_response_time
    );

    assert!(false, "{}", message);

}
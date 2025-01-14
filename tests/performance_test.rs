use std::net::TcpStream;
use std::io::{Write, Read};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
#[ignore = "Performance test is should give you an idea of the performance of your server uncomment to run it and remove the ignore ( AFTER FINISH IT FAILS TO GIVE YOU RESULTS )"]
fn performance_test() {
    // Number of concurrent clients
    let client_count = 4000; // Number of concurrent clients

    // Number of failed requests
    let failed_requests = Arc::new(Mutex::new(0));

    // Vector to store request durations
    let request_durations = Arc::new(Mutex::new(Vec::new()));

    // Start time
    let start_time = Instant::now();

    // Handles
    let mut handles = vec![];

    // Spawn threads
    for _ in 0..client_count {
        let failed_requests_clone = Arc::clone(&failed_requests);
        let request_durations_clone = Arc::clone(&request_durations);

        let handle = thread::spawn(move || {
            let start = Instant::now();
            let mut stream = match TcpStream::connect("localhost:8080") {
                Ok(s) => s,
                Err(_) => {
                    let mut failed = failed_requests_clone.lock().unwrap();
                    *failed += 1;
                    return;
                }
            };

            let request = "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n";
            if stream.write_all(request.as_bytes()).is_err() {
                let mut failed = failed_requests_clone.lock().unwrap();
                *failed += 1;
                return;
            }

            let mut response = String::new();
            if stream.read_to_string(&mut response).is_err() {
                let mut failed = failed_requests_clone.lock().unwrap();
                *failed += 1;
                return;
            }

            let duration = start.elapsed();
            let mut durations = request_durations_clone.lock().unwrap();
            durations.push(duration);
        });

        handles.push(handle);
    }

    // Insure that all threads are finished
    for handle in handles {
        handle.join().unwrap();
    }

    // End time
    let total_duration = start_time.elapsed();

    let total_requests = client_count as u64;

    // Calculate the number of successful requests
    let successful_requests = total_requests - *failed_requests.lock().unwrap();

    // Calculate the average response time
    let avg_response_time: f64 = {
        let durations = request_durations.lock().unwrap();
        durations.iter().map(|&d| d.as_secs_f64()).sum::<f64>() / successful_requests as f64
    };

    // Print the results
    let mut message  = "Performance Test Finished".to_string();
    message = message + "\nTotal Requests: " + &total_requests.to_string();
    message = message + "\nSuccessful Requests: " + &successful_requests.to_string();
    message = message + "\nFailed Requests: " + &*failed_requests.lock().unwrap().to_string();
    message = message + "\nPercentage of Successful Requests: " + &format!("{:.2}", successful_requests as f64 / total_requests as f64 * 100.0);
    message = message + "\nTotal Duration: " + &format!("{:.2}", total_duration.as_secs_f64());
    message = message + "\nRequests per Second: " + &format!("{:.2}", successful_requests as f64 / total_duration.as_secs_f64());
    message = message + "\nAverage Response Time (seconds): " + &format!("{:.4}", avg_response_time);
    assert!(false , "{}" , message);
}
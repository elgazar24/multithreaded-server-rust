
# Initial Implementation 

- Single threaded Server with limit of how many response could the thread server 
- Basic architecture with client and server, client handles requests and response which this is vague
- Hardcoded with no option for customization 
- Tests need to be fixed 



# New Implementation 

- Multi-threaded server handling multiple client connections.
- Implemented compatible SOLID principle and Clean Architecture.
- Basic Router implementation for managing the requests and send proper response.
- Worker to manage the threading mechanism 
- Worker Task to manage the task
- Ability to handle more than 4000 requests with Successful Requests 99.98 % and Average Response Time 0.7494 
- Ability to add an new features to all the components easily
- Clean, maintainable, and well-documented code
- Client implementation for sending HTTP requests and receiving responses.
- Automated tests for server functionality:
  - Performance test
  - Stress testing with many clients.
  - Handling invalid requests.
  - Handling raw messages and bounce them back to the client
  - Sending multiple requests to the server.
  - Simulating simultaneous client connections.
  - Server shutdown handling after processing requests.



## The New Architecture is Called Thread Pool 

After carefully read about available solutions, The Most suitable implementation is Thread Pool which enhance the efficiency and scalability of the multithreaded server. This design replaces the naive spawning of threads for every incoming connection with a structured, reusable pool of worker threads, which handle tasks dynamically. Below is a description of the architecture and its advantages.

Consist of three main components : 

 - Server Manager : 
    1. Start the server 
    2. Creating the workers 
    3. Start listening from the port to incoming TcpStreams 
    4. Assign tasks to an available worker and balance the load between them 
    5. Ensure to handle multiple requests concurrently while maintaining data consistency
    6. Allows for a clean shutdown of the server

 - Worker : 
    1. It's the thread itself
    2. Responsible for executing the tasks

 - WorkerTask : 
    1. it's the task which the worker will execute 


Also Added other component which is realated to handling the requset and response and some static files to mimic the real server operation : 
 - RequestManager :
    1. Act as the Parser and Router of the Server 
    2. Parses the request to extract the important details from it.
    3. Router to the needed function depend on the request 
    4. Generates the response like : 
        - Html , Css ,Js files :  verify file availabilty  , read the files 
        - images png , jpg verify file availabilty  , decode the images to raw data 
        - Error 404 page not found response 
        - Error 400 bad request 
        - Request is normal raw message it echo the message back to the client like the Initial Implementation
    5. Send the response to the client 

### Test Suite Results : 
      - Evidence (e.g., logs) that your server passes all tests on [Test_Suite_Results] Folder 
      - Additional test cases will be found in [tests] Folder 




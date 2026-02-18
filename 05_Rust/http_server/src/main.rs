pub mod model;
pub mod api;
pub mod handler;
 
use axum::{routing::{get, post, delete, put}, Router};
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::TcpListener, sync::RwLock};
use model::Student;
use api::{get_students, get_student, add_student, update_student, delete_student};
 
// Using RwLock instead of Mutex for better concurrent read access
type SharedState = Arc<RwLock<Vec<Student>>>;

#[tokio::main]
async fn main() {
    println!("=== Multithreaded Student Management Server ===");
    println!("Main thread: {:?}", std::thread::current().id());
    
    // Load student data from a file and initialize the shared state
    let students = handler::load_students();
    println!("Loaded {} students from file", students.len());

    // Wrap the students vector in a RwLock and Arc
    // RwLock allows multiple concurrent readers or one writer
    let state = Arc::new(RwLock::new(students));
 
    println!("\n=== Available API Endpoints ===");
    println!("GET    http://127.0.0.1:4500/students          - Get all students");
    println!("GET    http://127.0.0.1:4500/students/{{id}}     - Get student by ID");
    println!("POST   http://127.0.0.1:4500/students          - Add new student");
    println!("PUT    http://127.0.0.1:4500/students/{{id}}     - Update student");
    println!("DELETE http://127.0.0.1:4500/students/{{id}}     - Delete student");
    
    println!("\n=== Example cURL Commands ===");
    println!("# Get all students:");
    println!("curl -X GET http://127.0.0.1:4500/students\n");
    
    println!("# Add new student:");
    println!("curl -X POST http://127.0.0.1:4500/students -H \"Content-Type: application/json\" -d '{{\"name\":\"John Doe\",\"email\":\"john@example.com\",\"mobile\":\"1234567890\"}}'\n");
    
    println!("# Update student (replace {{id}} with actual ID):");
    println!("curl -X PUT http://127.0.0.1:4500/students/{{id}} -H \"Content-Type: application/json\" -d '{{\"name\":\"John Updated\",\"email\":\"john@example.com\",\"mobile\":\"1234567890\"}}'\n");
    
    println!("# Delete student (replace {{id}} with actual ID):");
    println!("curl -X DELETE http://127.0.0.1:4500/students/{{id}}\n");
 
    // Build the router with method-specific routing
    let app = Router::new()
        .route("/students", get(get_students).post(add_student))
        .route("/students/:id", 
            get(get_student)
            .put(update_student)
            .delete(delete_student)
        )
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 4500));
    println!("=== Server Status ===");
    println!("Server starting at http://{}", addr);
    println!("Each HTTP method (GET, POST, PUT, DELETE) will run on separate threads");
    println!("Waiting for requests...\n");

    // Create a TCP listener
    let listener = TcpListener::bind(addr).await.unwrap();

    // Run the server
    axum::serve(listener, app)
        .await
        .unwrap();
}
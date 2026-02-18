use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::model::Student;

/// get all students from the file and store them in the vector 
pub fn load_students() -> Vec<Student> {
    fs::read_to_string("students.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| vec![]))
        .unwrap_or_else(|_| vec![])
}

/// save the vector of students to the file - thread-safe version
pub async fn save_students_async(students: &[Student]) {
    // Use tokio::spawn to run blocking file I/O in a separate thread
    let students_json = serde_json::to_string_pretty(students).unwrap();
    
    tokio::task::spawn_blocking(move || {
        fs::write("students.json", students_json).unwrap();
        println!("[THREAD: {:?}] File saved successfully", std::thread::current().id());
    })
    .await
    .unwrap();
}

/// Thread-safe file saving with explicit thread info
pub async fn save_students_with_thread_info(students: Arc<RwLock<Vec<Student>>>, operation: &str) {
    let students_read = students.read().await;
    let students_clone = students_read.clone();
    let operation = operation.to_string();
    
    tokio::task::spawn_blocking(move || {
        let json = serde_json::to_string_pretty(&students_clone).unwrap();
        fs::write("students.json", json).unwrap();
        println!("[{}] Saved by thread: {:?}", operation, std::thread::current().id());
    })
    .await
    .unwrap();
}
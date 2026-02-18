use axum::{extract::{Path, State}, http::StatusCode, response::Json};
use uuid::Uuid;
use crate::{model::Student, handler::save_students_async, SharedState};
use std::sync::Arc;

/// GET all students - Runs in a dedicated thread
/// curl -X GET http://127.0.0.1:4500/students
pub async fn get_students(State(state): State<SharedState>) -> Json<Vec<Student>> {
    // Spawn in a separate task/thread
    let handle = tokio::spawn(async move {
        let students = state.read().await;
        let result = students.clone();
        println!("[GET ALL] Executed on thread: {:?}", std::thread::current().id());
        result
    });
    
    let students = handle.await.unwrap();
    Json(students)
}

/// GET a student by id - Runs in a dedicated thread
/// curl -X GET http://127.0.0.1:4500/students/{id}
pub async fn get_student(
    Path(id): Path<String>, 
    State(state): State<SharedState>
) -> Result<Json<Student>, StatusCode> {
    // Spawn in a separate task/thread
    let handle = tokio::spawn(async move {
        let students = state.read().await;
        println!("[GET BY ID] Executed on thread: {:?}", std::thread::current().id());
        students.iter()
            .find(|s| s.id == id)
            .cloned()
            .ok_or(StatusCode::NOT_FOUND)
    });
    
    let result = handle.await.unwrap();
    result.map(Json)
}

/// POST - Add a new student - Runs in a dedicated thread
/// curl -X POST http://127.0.0.1:4500/students -H "Content-Type: application/json" -d "{ \"name\": \"Aman\", \"email\": \"aman@example.com\", \"mobile\": \"9876543210\" }"
pub async fn add_student(
    State(state): State<SharedState>, 
    Json(mut student): Json<Student>
) -> StatusCode {
    // Clone state for the spawned task
    let state_clone = Arc::clone(&state);
    
    // Spawn in a separate task/thread
    let handle = tokio::spawn(async move {
        student.id = Uuid::new_v4().to_string();
        
        {
            let mut students = state_clone.write().await;
            students.push(student.clone());
            println!("[POST] Executed on thread: {:?}", std::thread::current().id());
        }
        
        // Save to file in another thread
        let students = state_clone.read().await;
        save_students_async(&students).await;
        
        StatusCode::CREATED
    });
    
    handle.await.unwrap()
}

/// PUT - Update a student - Runs in a dedicated thread
/// curl -X PUT http://127.0.0.1:4500/students/{id} -H "Content-Type: application/json" -d "{ \"name\": \"Aman Verasia\", \"email\": \"aman@example.com\", \"mobile\": \"9876543210\" }"
pub async fn update_student(
    Path(id): Path<String>, 
    State(state): State<SharedState>, 
    Json(updated_student): Json<Student>
) -> StatusCode {
    let state_clone = Arc::clone(&state);
    
    // Spawn in a separate task/thread
    let handle = tokio::spawn(async move {
        let result = {
            let mut students = state_clone.write().await;
            println!("[PUT] Executed on thread: {:?}", std::thread::current().id());
            
            if let Some(student) = students.iter_mut().find(|s| s.id == id) {
                student.name = updated_student.name;
                student.email = updated_student.email;
                student.mobile = updated_student.mobile;
                true
            } else {
                false
            }
        };
        
        if result {
            // Save to file in another thread
            let students = state_clone.read().await;
            save_students_async(&students).await;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    });
    
    handle.await.unwrap()
}

/// DELETE a student - Runs in a dedicated thread
/// curl -X DELETE http://127.0.0.1:4500/students/{id}
pub async fn delete_student(
    Path(id): Path<String>, 
    State(state): State<SharedState>
) -> StatusCode {
    let state_clone = Arc::clone(&state);
    
    // Spawn in a separate task/thread
    let handle = tokio::spawn(async move {
        let found = {
            let students = state_clone.read().await;
            students.iter().any(|s| s.id == id)
        };
        
        if found {
            {
                let mut students = state_clone.write().await;
                students.retain(|s| s.id != id);
                println!("[DELETE] Executed on thread: {:?}", std::thread::current().id());
            }
            
            // Save to file in another thread
            let students = state_clone.read().await;
            save_students_async(&students).await;
            StatusCode::OK
        } else {
            StatusCode::NOT_FOUND
        }
    });
    
    handle.await.unwrap()
}
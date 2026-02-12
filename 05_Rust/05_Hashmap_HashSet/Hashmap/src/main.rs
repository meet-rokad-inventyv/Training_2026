use std::collections::HashMap;

#[derive(Debug)]
struct Student {
    id: u32,
    name: String,
    marks: u32,
}

fn main() {

    let mut students: HashMap<u32, Student> = HashMap::new();


    if let Err(e) = students.try_reserve(2) {
        println!("Memory reservation failed: {}", e);
    }

    students.insert(
        1,
        Student {
            id: 1,
            name: String::from("Alice"),
            marks: 85,
        },
    );

    students.insert(
        2,
        Student {
            id: 2,
            name: String::from("Bob"),
            marks: 40,
        },
    );

   
    let s1 = students.get(&1).unwrap();
    println!("Student 1: {:?}", s1);
 
    let mut optional_student = students.remove(&2);
    let taken_student = optional_student.take();

    if let Some(stu) = taken_student {
        println!("Taken student: {:?}", stu);
    }
    println!("{:?}",optional_student);


    students.retain(|_, student| student.marks >= 50);
 
    let mut new_students = HashMap::new();
    new_students.insert(
        3,
        Student {
            id: 3,
            name: String::from("Charlie"),
            marks: 90,
        },
    );
    
    students.extend(new_students);
    println!("{:?}",new_students);

}

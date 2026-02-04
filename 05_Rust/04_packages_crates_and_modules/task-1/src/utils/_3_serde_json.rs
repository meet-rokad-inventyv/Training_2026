use serde_json;
use serde::{Deserialize, Serialize};

#[derive(Debug,Serialize,Deserialize)]
struct Marks {
    maths : i32,
    chemistry :i32,
    physics : i32
}

#[derive(Debug,Serialize,Deserialize)]
struct Student
{
    name : String,
    id : i32,
    email : String,
    phone_number : String,
    grade : char,
    marks : Marks
}

pub fn main()
{
    let student1 = Student 
    {
        name : String::from("Meet"),
        id : 1,
        email : String::from("meet@gmail.com"),
        phone_number : String::from("+91 7569046534"),
        grade : 'A',
        marks : Marks{
            maths : 86,
            chemistry : 84,
            physics : 89
        }
    };

    //First serialize and deserialize 

    let json_string2 = serde_json::to_string(&student1).unwrap();
    let student : Student= serde_json::from_str(&json_string2).unwrap();

    println!("{}",student.name);
    println!("{}",student.id);
    println!("{}",student.email);
    println!("{}",student.phone_number);
    println!("{:?}",student.marks);


    //deserialize using raw string literal

    let json_string1 = r#"{"name":"Meet","id":1,"email":"meet@gmail.com","phone_number":"+91 7569046534","grade":"A","marks":{"maths":86,"chemistry":84,"physics":89}}"#;
    let student : Student= serde_json::from_str(json_string1).unwrap();

    println!("{}",student.name);
    println!("{}",student.id);
    println!("{}",student.email);
    println!("{}",student.phone_number);
    println!("{:?}",student.marks);
}
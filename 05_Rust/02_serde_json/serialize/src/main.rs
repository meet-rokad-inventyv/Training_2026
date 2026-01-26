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

fn main()
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
    
    let json_string = serde_json::to_string(&student1).unwrap();

    println!("{}",json_string);
}
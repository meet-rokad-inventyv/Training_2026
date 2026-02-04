#[derive(Debug)]
struct Marks {
    maths : i32,
    chemistry :i32,
    physics : i32
}

struct Student
{
    name : String,
    id : i32,
    email : String,
    phone_number : String,
    grade : char,
    marks : Marks
}

//implementation of Student structure
impl Student 
{
    fn set_name(&mut self,name:String)
    {
        self.name=name;
    }
    fn get_name(&self)->&String
    {
        &self.name
    }

    fn set_id(&mut self,id:i32)
    {
        self.id = id;
    }

    fn get_id(&self)->i32
    {
        self.id
    }

    fn set_email(&mut self,email:String)
    {
        self.email = email;
    }

    fn get_email(&self)->&String
    {
        &self.email
    }
    
    fn set_phone_number(&mut self,phone_number:String)
    {
        self.phone_number = phone_number;
    }

    fn get_phone_number(&self)->&String
    {
        &self.phone_number
    }


    fn set_grade(&mut self,grade:char)
    {
        self.grade = grade;
    }

    fn get_grade(&self)->char
    {
        self.grade
    }

    fn get_marks(&self)->Marks
    {
        Marks{
            maths:self.marks.maths,
            chemistry:self.marks.chemistry,
            physics : self.marks.physics
        }
    }

    fn get_student_info(&self)->String
    {
        format!("Name:{}  ID:{}  email:{}  phone_number:{}  Grade:{}  Marks:{:?}",self.name,self.id,self.email,self.phone_number,self.grade,self.marks)
    }

    fn get_student_info_with_args(&self,name:&str,id:i32,email:&str,phone_number:&str,grade:char,marks:&Marks)-> String
    {
        format!{"Name:{}  ID:{}  email:{}  phone_number:{}  Grade:{}  Marks:{:?}",name,id,email,phone_number,grade,marks}
    } 
}

//implementation of Marks structure which is inside Student structure
impl Marks 
{
    fn set_maths(&mut self, maths: i32) {
        self.maths = maths;
    }

    fn set_chemistry(&mut self, chemistry: i32) {
        self.chemistry = chemistry;
    }

    fn set_physics(&mut self, physics: i32) {
        self.physics = physics;
    }

    fn get_maths(&self) -> i32 {
        self.maths
    }

    fn get_chemistry(&self) -> i32 {
        self.chemistry
    }

    fn get_physics(&self) -> i32 {
        self.physics
    }
}


pub fn main()
{
    let mut student1 = Student 
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
    
    //print structure

    println!("{}",student1.get_name());
    println!("{}",student1.get_id());
    println!("{}",student1.get_email());
    println!("{}",student1.get_phone_number());
    println!("{}",student1.get_grade());
    println!("{}",student1.marks.get_maths());
    println!("{}",student1.marks.get_chemistry());
    println!("{}",student1.marks.get_physics());
    println!("{:?}",student1.get_marks());
    println!("{}",student1.get_student_info());
    println!("{}",student1.get_student_info_with_args(&student1.name,student1.id,&student1.email,&student1.phone_number,student1.grade,&student1.marks));


    //Update structure

    student1.set_name(String::from("Sujal"));
    student1.set_id(2);
    student1.set_email(String::from("sujal@gmail.com"));
    student1.set_phone_number(String::from("+91 7846536784"));
    student1.set_grade('B');
    student1.marks.set_maths(70);
    student1.marks.set_chemistry(71);
    student1.marks.set_physics(73);


    //print structure after updation

    println!("{}",student1.get_name());
    println!("{}",student1.get_id());
    println!("{}",student1.get_email());
    println!("{}",student1.get_phone_number());
    println!("{}",student1.get_grade());
    println!("{}",student1.marks.get_maths());
    println!("{}",student1.marks.get_chemistry());
    println!("{}",student1.marks.get_physics());
    println!("{:?}",student1.get_marks());
    println!("{}",student1.get_student_info());
    println!("{}",student1.get_student_info_with_args(&student1.name,student1.id,&student1.email,&student1.phone_number,student1.grade,&student1.marks));
}
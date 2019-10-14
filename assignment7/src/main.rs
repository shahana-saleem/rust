#[derive(Debug)]
struct Student {
    name: String,
    age:u8,
    grade: String,
    percentage: f32,
}

impl Student {
    fn pers(&self){        
        println!("The percentage of Student = {}%",self.percentage); 
    }

    fn new(name:String,age:u8,grade:String,percentage:f32)-> Student{
    Student {
       name,
       age,
       grade,
       percentage,
    }
}
}

fn main () {
    
    let std = Student::new(String::from("asfia"),9,String::from("fourth"),95.0);
    println!("{:#?}",std);
    std.pers();
}   
    
    // println!("The name of first student is {} and the name of second student is {}. ",std.name,std1.name); 



//name:String,age:u8,grade:String,percentage:f32

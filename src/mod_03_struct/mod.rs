// Data Struct
#[derive(Debug)]
struct Data{
    id: i32,
    name: String,
    value: f64,
}

// this is for impl funcs for Data structure
impl Data{
    fn print(&self){
        println!("id: {}, name: {}, value: {}", self.id, self.name, self.value);
    }
    fn test(&self){
        println!("test = {}", self.id);
    }
}


pub fn run(){
    println!("===== mod_03_struct::run() =====");
    // String::from("abner") and "abner".to_string() are allocate memory by heap
    let data = Data {id: 10, name:String::from("abner"), value:10.0};
    let data2 = Data {id: 20, name:"abner".to_string(), value:20.0};
    let mut data3 = Data {id: 20, name:"abner".to_string(), value:20.0};
    data.print();
    data.test();

    data3.id = 100;

    // becasue of #[derive(Debug)], that we can print it directly
    println!("{:?}", data2);

    println!("");
}
trait Base {
    fn run(&self){
        println!("Basic::run()");
    }

    fn say(&self);
}

struct Cat{
    name:String
}

impl Base for Cat{
    fn run(&self){
        println!("I'm a Cat, named {}", self.name);
    }
    fn say(&self){
        println!("Meow");
    }
}

struct Dog{
    name:String
}

impl Base for Dog{
    fn run(&self){
        println!("I'm a Dog, named {}", self.name);
    }
    fn say(&self){
        println!("Woof");
    }
}

pub fn run(){
    println!("===== mod_05_trait::run() =====");

    let animal = Dog{name:"abner".to_string()};
    animal.run(); 

    let animal = Cat{name:"abner222222".to_string()};
    animal.run(); 


    let animals: Vec<Box<dyn Base>> = vec![
        Box::new(Dog{name:"abner".to_string()}),
        Box::new(Cat{name:"abner222222".to_string()})
    ];

    for animal in animals{
        animal.run();
        animal.say();
    }
    
    println!();
}
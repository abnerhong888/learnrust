

fn parent_func1() -> i32{
    return 10
}

fn parent_func2() -> i32{ 
    100
}

pub mod level1{
    // bring previous every to level1
    use super::*;

    pub fn level1_func1(){
        println!("level1_func1() {}, {}", parent_func1(), parent_func2());
    }


    pub mod level2{
        pub mod level3{
            pub fn level3_func1(){
                println!("level3_func1() {}, {}", super::super::parent_func1(), super::super::parent_func2());
            }
            
        }
    }
}
pub fn run(){
    println!("===== mod_07_namespace::run() =====");

    level1::level1_func1();
    level1::level2::level3::level3_func1();


    println!("");
}
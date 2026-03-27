use crate::mod_01_module;


pub fn run(){
    println!("===== mod_06_crate::run() =====");

    crate::mod_01_module::func1();
    mod_01_module::func2();
    println!();
}
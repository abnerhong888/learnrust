// need to declare here in order to make it visiable
pub mod test; 
pub fn run(){
    println!("===== mod_01_module::run() =====");
    println!("this is mod_01::run()");
}

#[inline]
pub fn func1(){
    println!("this is mod_01::func1()");
}

pub fn func2(){
    println!("this is mod_01::func2()");
}
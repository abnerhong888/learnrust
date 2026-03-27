
static GLOBAL_VAR: i32 = 10;
// &str like const char* in c
static GLOBAL_STR: &str = "abner";

// heap allocated string once
use std::sync::LazyLock;
static GLOBAL_STR2: LazyLock<String> = LazyLock::new(|| {
    String::from("abner2")
});

#[allow(unused)]
type INT = i32;

pub fn run(){
    println!("===== mod_02_basic::run() =====");
    const V:i32 = 10;
    println!("const i32 var = {}", V);
    println!("global i32 var = {}", GLOBAL_VAR);
    println!("global string var = {}", GLOBAL_STR);
    println!("global string var2 = {}", *GLOBAL_STR2);
    println!();
}
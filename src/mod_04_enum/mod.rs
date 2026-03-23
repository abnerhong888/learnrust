enum Color{
    Red,
    #[allow(dead_code)]
    Green,
    #[allow(dead_code)]
    Blue
}

enum Info {
    Name(String),
    Age(u8),
    Id(u32)
}

fn check_color(color: &Color){
    print!("color is ");
    match color{
        Color::Red => println!("[red]"),
        Color::Green => println!("[green]"),
        Color::Blue => {
            println!("[blue]")
        }
    }
}

fn check_info(info: &Info){
    print!("info is ");
    match info{
        Info::Name(name) => println!("[name: {}]", name),
        Info::Age(age) => println!("[age: {}]", age),
        Info::Id(id) => println!("[id: {}]", id)
    }
}

pub fn run(){
    println!("===== mod_04_enum::run() =====");

    let color = Color::Red;
    let info = Info::Name("abner".to_string());
    // for ignore warning
    let _ = Info::Age(0);
    let _ = Info::Id(0);

    check_color(&color);

    check_info(&info);
    
    println!("");
}
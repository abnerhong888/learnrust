pub fn run(){
    println!("===== mod_09_some::run() =====");

    // some is one of two variable of the Option enum

    let name: Option<&str> = Some("Abner");
    let no_name: Option<&str> = None;

    println!("name = {:?}", name);
    println!("no_name = {:?}", no_name);

    match name {
        Some(its_name) => println!("its_name = {}", its_name),
        None => println!("no name") 
    }

    // if None go else
    if let Some(its_name) = name {
        println!("its_name = {}", its_name);
    } else {
        println!("no name");
    }

    let mut colors = vec!["red", "green", "blue"];
    
    while let Some(color) = colors.pop(){
        println!("color = {}", color);
    }

    // this method is not recommend because it is unsafe
    let get_name:&str = name.unwrap();
    let get_no_name:&str = no_name.unwrap_or("Alan");
    println!("get_name = {}", get_name);
    println!("get_no_name = {}", get_no_name);

    println!();
}
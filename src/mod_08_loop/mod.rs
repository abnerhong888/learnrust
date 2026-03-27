pub fn run(){
    println!("===== mod_08_loop::run() =====");

    for i in 1..5 {
        print!("{}, ", i);
    }
    println!();

    for i in 1..=5 {
        print!("{}, ", i);
    }
    println!();

    let mut list = [1, 2, 3, 4, 5];

    // i = i32 
    for i in list {
        print!("{}, ", i);
    }
    println!();

    // i = &i32 is a pointer
    for i in list.iter() {
        print!("{}, ", i);
    }
    println!();

    // i = &mut i32 is a pointer
    for i in list.iter_mut() {
        *i += 1;
        print!("{}, ", i);
    }
    println!();

    // i = i32 
    for i in list.into_iter() {
        print!("{}, ", i);
    }
    println!();

    let colors = ["red", "green", "blue"];
    for (index, color) in colors.iter().enumerate() {
        println!("{}: {}", index, color);
    }

    let mut number = 3;

    while number != 0 {
        print!("{}, ", number);
        number -= 1;
    }
    println!();

    let result = loop {
        number += 1;
        if number == 10 {
            break number * 2;
        }
    };
    println!("result = {}", result);
    
    'outer: loop {
        println!("Entered outer");
        loop {
            println!("Entered inner");
            break 'outer; // Exits both loops immediately
        }
    }
    
    println!();
}
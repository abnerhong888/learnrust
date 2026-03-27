use std::collections::HashMap;

macro_rules! add{
    ($a:expr, $b:expr) =>{
        $a + $b
    }
}

macro_rules! create_struct {
    ( $name:ident, $type: tt) => {
        struct $name{
            value: $type,
        }
    };
}

macro_rules! impl_struct {
    ( $struct:ty) =>{
        impl $struct{
            fn print(&self){
                println!("name = {}, value = {}", std::any::type_name::<$struct>(), self.value);
            }
        }
    }
}

create_struct!(MyStruct, i32);
impl_struct!(MyStruct);

// $(...),* for auto expand by ','
macro_rules! map {
    ( $( $key:expr => $value:expr),*) => { 
        {
            let mut _map = HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

macro_rules! myvec {
    ( $($value:expr),*) =>{
        {
            let mut _vec = Vec::new();
            $(
                _vec.push($value);
            )*
            _vec
        }
    }
}

macro_rules! create_struct2 {
    ($name:ident { $( $field:ident : $ty:ty ),* }) => {
        #[derive(Debug)]
        struct $name {
            $( 
                #[allow(unused)]
                pub $field: $ty, 
            )*
        }
    };
}

create_struct2!(User {
    id: i32,
    username: String,
    is_active: bool
});

pub fn run(){
    println!("===== mod_11_macro::run() =====");

    let sum = add!(1, 2);
    println!("sum = {}", sum);

    let my_struct = MyStruct{value: 10};
    my_struct.print();

    let u = User { id: 1, username: "Alice".into(), is_active: true };
    println!("{:?}", u);

    let scores = map!(
        "Alice" => 10,
        "Bob" => 20,
        "Charlie" => 30
    );

    println!("{:?}", scores);


    let values = myvec!(1, 2, 3, 4, 5);
    println!("{:?}", values);


    println!();
}
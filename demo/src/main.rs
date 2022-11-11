use std::collections::HashMap;

#[derive(Debug)]
struct Rectangle{
    width: u32,
    length: u32,
} 

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }    
}

fn printcol() {
    let mut map=HashMap::new();
    for i in 0..10{
        map.insert(i,i.to_string()+"str");
    }
    for i in map.iter(){
        println!("{:?}",i);
    }
}


fn main() {
    let ret=Rectangle{
        width:30,
        length:50,
    };
    println!("{}",ret.area());
    println!("{:#?}",ret);   
    printcol();

}


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

fn main() {
    let ret=Rectangle{
        width:30,
        length:50,
    };
    println!("{}",ret.area());
    println!("{:#?}",ret);   
}

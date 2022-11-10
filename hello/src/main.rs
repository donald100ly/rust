
use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {

    println!("猜数！");

    let secret_number=rand::thread_rng().gen_range(1..=100);
    
    println!("神秘数字是：{}",secret_number);
    
    loop {

    println!("猜一个数");

    
    

    let mut guess=String::new();

    io::stdin().read_line(&mut guess).expect("无法读取");
    
    let guess:u32=match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>continue,
    };
    
    println!("你猜的数字是：{}",guess);

    match guess.cmp(&secret_number) {
        Ordering::Less    => println!("Too small!"),
        Ordering::Greater => println!("Too Big!"),
        Ordering::Equal   =>{
            println!("You win!");
            break;
        } 
    }
        
    }

    

}

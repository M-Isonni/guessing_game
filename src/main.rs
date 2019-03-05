use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1,101);    

    loop{
        println!("please input a number");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
        .expect("unable to read the numeber");
        

        let guess: u32 = match guess.trim().parse() {
            Ok(num)=>num,
            Err(_)=>{println!("type a number"); continue;},
        };


        println!("your guess: {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less=>println!("too small"),
            Ordering::Greater=>println!("too great"),
            Ordering::Equal=>{println!("win"); break;}
        }

    }

}


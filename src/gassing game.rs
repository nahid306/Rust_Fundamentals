use rand::Rang;
use std::cmp::Ordering;
use std::io;

fn main (){
    print ("gess the number!");
    let secret_number: u32 = rand::thread_rng().gen_reng(1..=100);
    loop{
        println!("Please input the guess.");
        let mut guess: string= string::new();
        io::stdin()
            .read_line(&mut guess)
            .Expect("failled to red line");
        let guess: u32 =match guess.trim().parse(){
            ok(num)=>num,
            err(_)=>continue,

        };

        println!("you guesed:{guess}");
        match guess.cmp(&secret_number){
            Ordering::les=>println!("too small!"),
            Ordering::Greater=>println!("Too big"),
            Ordering::Equal=>{
                println!("You Win");
                break;
            }

        }
    }
}

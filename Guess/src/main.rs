use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main(){
    println!("Welcome to the gussing game");

    let secret_num=rand::thread_rng().gen_range(1..100);

    // loop game to keep ask user untill get it right
    
   loop{
    // user input
    let mut guess=String::new();
    println!("gusse a number between 1 ... 100 : ");
    io::stdin()
    .read_line(&mut guess ) 
    .expect("can't read input");

    // check that type will be same as secretnum
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("you gussed {guess}");
    if guess>= 1 && guess <= 100 {
        match guess.cmp(&secret_num){
            Ordering::Equal=>
                {
                    println!("YOU WIN !!!");
                    break;
                },
            Ordering::Greater=>println!("down more "),
            Ordering::Less=>println!("up more ")
        }
    }
    else{
        println!("enter only numbers between 1 to 100")
    }
   
   }


}
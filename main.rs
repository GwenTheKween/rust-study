//================================
//Library importing
use std::io;

//================================
//readability stuff for new rust user

fn integer_input() -> i32{
    let mut line = String::new(); //result of reading from stdin

    io::stdin().read_line(&mut line)
        .ok().expect("unable to read line");

    let ret: i32 = line.trim().parse()
        .ok().expect("please enter a number");
    ret
}

//================================
//Actual code
fn main(){
    println!("Henlo uordi! Enter a number");
    let num = integer_input();
    println!("you entered {}", num);
}

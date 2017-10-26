fn main(){

    #[macro_use] extern crate text_io;

//testing
    let mut s = String::new();
    scan!("{} is your name", s);
    let mut age = i32;
    scan!("{} is your age", age);
    if age >= 18{
        println!("You can vote.");
        println!("What party would you like to vote for?");
        let mut partychoice = String::new();
            if partychoice == "LAB"{
                println! ("Voting for Labour");
            }else if partychoice == "CON"{
                println! ("Voting for the Tories");
            }else{
                println("Party not recongised!");
            }

    }else{
        println!("Too young to vote");
    }


}
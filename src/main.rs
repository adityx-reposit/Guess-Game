

// // fn main() {
// //     let x1=recieved();
// //     let x2=String::from("hello");
// //     let x3=confirm(x2);
// // println!("value of x here in main is {}",x1);
// // println!("{}",x3);
// // }


// // fn recieved()->String{
// //     let new_string = String::from("world");
// //     return new_string;
// // }


// // fn confirm(recieved:String)->String{
// //     return recieved;
// // }

// // fn main(){
// //     let s1=String::from("aditya");
// //     let (s2,len) =calculate_len(s1);
// //     println!("length of {} is {}",s2,len);
// // }


// // fn calculate_len(s:String)->(String,usize){
// //      let length=s.len();
// //      return (s,length);
// // }



// // taking input from user
// use std::io;
// fn main(){
//   let mut input =String::new();
//   println!("Please Enter Your name");
//   io::stdin()
//   .read_line(&mut input)
//   .expect("Failed to read the line");
//   println!("input string is {}",input);


  
// }
use rand::{ rng, Rng};
use std::{io, string};
use colored ::Colorize;
fn take_input()->usize{
  let mut input =String::new();
  io::stdin()
  .read_line(&mut input)
  .unwrap();
return input.trim().parse().expect("enter a valid number");

}


fn main(){
  println!("{}","Enter the Number To Be Checked".blue());
  compare();
}


fn generate()->usize{
  let mut rng = rand::rng();
   let n1=rng.random_range(1..=10);
    return n1;
}

fn compare(){
  let real_number= generate();
  let guessed=take_input();
  if real_number== guessed{
     println!("{}","Number matched Congrats".green());
  }
  println!("{}","You are wrong try again".red());
  print!("real number was {}",real_number.to_string().green());
}
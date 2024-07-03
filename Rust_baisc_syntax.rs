// conditional loops in Rust
// To uncomment any area => cmd + /    or   ctrl + /  
// fn main(){
    
//     //iterating over multiple words in sentence (String)
//     let sentence = String::from("myname my name is , ...Ishaaaan");
//     let first_word = get_first_word(sentence);
//     println!(" FIrst word is : {}",first_word);
// }
// //like TS return type using arrow
// fn get_first_word(sentence:String)-> String {
//     let mut ans = String::from("");
//     for char in sentence.chars(){
//         if char == ' ' {
//             break;
//         }
//         ans.push(char)
//     }
//     return ans; 
// }




fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Let's galvanate the rust");

// Syntax now similar to C

// let is_even = false;
    // // IF-ELSE loop

    // if is_even{
    //     println!("is even");
    // }else if !is_even{
    //     println!("is not even");
    // }

    // //for loop

    // for i in 0..10{
    //     println!("i : {}",i);

    // }
}

// Type identifying

// fn main() {
//     println!("let's go ");
//     let x: i16 = 56;
//     let y: u32 = 6969;
//     let z: f32 = 877.001;

//     println!(" x : {} ,y : {} ,z: {}  ",x, y ,z)

// }

//Integer and for loop
// fn main(){
//     let mut x = 10;

//     for i in 0..100 {
//         x = x +100;
//     }
//     println!("x = {}", x);
// }

//boolean uses
// fn main(){
//     let is_male = false;
//     let is_above_18 = true;

//     if is_male{
//         println!("You're a male");
//     }
//     else {
//         println!("you're not a male ");
//     }
//     if is_male && is_above_18{
//         println!("You're of legal age male");
//     }else {
//         println!("Male Not of legal Age");
//     }
// }


// stirngs and uses '&str', 'String'

// fn main(){
//     //let greeting ="Namste World";
//     let greeting = String::from("hello World");
//     println!("{}",greeting);
    

//     let char1 = greeting.chars().nth(12);
    
//     match char1{
//         Some(c )=> println!("{}",c),
//         None => println!("No Character at Index 12"),
//     }
//      println!("char1: {}",char1.unwrap());
// }
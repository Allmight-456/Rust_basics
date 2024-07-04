//To run a rust file , Compile=> rustc path_to_FIle/file.rs
// Run => ./file


// conditional loops in Rust
// To uncomment any area => cmd + /    or   ctrl + /  

//Stack and heap concept in Rust
// //stacks are used for immutable objects  , are faster
// // Heaps are mostly for mutable capacity and not determinable capacity and other scenarios as well

// fn main(){
//     stack_function();
//     heap_function();
//     update_string();
    
// }

// fn stack_function(){
//     let a=20;
//     let b=10;
//     let c = a+b;
//     println!("Stack function : Sum of a :{} and  b :{} is c :{}",a ,b,c);
// }

// fn heap_function(){
//     let s1 = String::from("Polo");
//     let s2 = String::from("Singh");
//     let combined = format!("{} {}",s1,s2);
//     println!("Heap function when combined says : {}", combined);
// }
// fn update_string(){
//     let mut s= String::from("Intially called polo");
//     println!("{}",s);
//     s.push_str(" ,Now known as Shabdansh ");
//     println!("{}",s);

//     //this is part where all three values that are stored in Stack to locate heap is given
//     // we will see How Pointer , length  and Capacity are influenced .

//     println!("Length : {}  Capacity:{}  Pointer:{:p}",s.len(),s.capacity(),s.as_ptr());

//     for _i in 0..10{
//         s.push_str("and some additional text");
//         println!("After push => Length : {}  Capacity:{}  Pointer:{:p}",s.len(),s.capacity(),s.as_ptr());
//     }
    
// }

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


//  Option enum is used for null  data types as null is not an accepted data type in rust
//this how it would look ,  but it is predefined like Result so no need to define it as data type
#[allow(dead_code)]
pub enum MyOption<T> {
    Some(T),
    None,
}

// two ways to go about it  1.Option
fn first_letter_a(s:String) -> Option<i32>{
    for (index , character ) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

fn main(){
    let my_string = String::from("Ishan");
    let res = first_letter_a(my_string);
    match res {
        Some(index) => print!("The first occurence of 'a' is at index {}", index),
        None => print!("There is no 'a' in this String."),
    }
}


// second way to use Result<T , E>

#[allow(dead_code)]
fn first_letter_a_result_enum(s:String) -> Result<i32,String>{
    for (index , character ) in s.chars().enumerate() {
        if character == 'a' {
            return Ok(index as i32);
        }
    }
    return Err("No such letter present".to_string());             //it is an &str so change it to String
}
#[allow(dead_code)]
fn _main(){
    let my_string = String::from("Ishan");
    let res = first_letter_a_result_enum(my_string);
    match res {
        Ok(index) => print!("The first occurence of 'a' is at index {}", index),
        Err(_err ) => print!("There is no 'a' in this String."),
    }
}
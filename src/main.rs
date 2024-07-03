//Stack and heap concept in Rust
//stacks are used for immutable objects  , are faster
// Heaps are mostly for mutable capacity and not determinable capacity and other scenarios as well

fn main(){
    stack_function();
    heap_function();
    update_string();
    
}

fn stack_function(){
    let a=20;
    let b=10;
    let c = a+b;
    println!("Stack function : Sum of a :{} and  b :{} is c :{}",a ,b,c);
}

fn heap_function(){
    let s1 = String::from("Polo");
    let s2 = String::from("Singh");
    let combined = format!("{} {}",s1,s2);
    println!("Heap function when combined says : {}", combined);
}
fn update_string(){
    let mut s= String::from("Intially called polo");
    println!("{}",s);
    s.push_str(" ,Now known as Shabdansh ");
    println!("{}",s);

    //this is part where all three values that are stored in Stack to locate heap is given
    // we will see How Pointer , length  and Capacity are influenced .
    
    println!("Length : {}  Capacity:{}  Pointer:{:p}",s.len(),s.capacity(),s.as_ptr());

    for _i in 0..10{
        s.push_str("and some additional text");
        println!("After push => Length : {}  Capacity:{}  Pointer:{:p}",s.len(),s.capacity(),s.as_ptr());
    }
    
}
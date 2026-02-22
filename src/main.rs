fn main(){
    let num:u8 =7;
    println!("The numbner is {}",num);

    // String 
    let mut string_literal= String::from("Hello, Gamers");
    string_literal.push_str("BEST GAME EVER");
    println!("{}",string_literal);

    // Tupple

    let emp_info:(&str,u8)=("Rishabh",18);
    let emp_name=emp_info.0;
    let emp_age=emp_info.1;

    // destruncturing
    let (emp_name,emp_age)=emp_info;
    println!("Employee name is {} and age is {}",emp_name,emp_age);

    println!("Employee name is {} and age is {}",emp_name,emp_age);
     //  Function Call!
     let num1:u8=69;
     let num2:u8=31;
     let result:u8=num1+num2;
     println!("The Total Sum is {}",result);


     // Variable scoping 

     let outside:u8=1;
     {
        let inside:u8=2;
        println!("The inside value is {}",inside);
     }

     println!("The outside value is {}",outside);


     // Ownership
     let str1 = String::from("Rishabh");
     let str2 = str1;
     println!("The str2 = {}",str2);

     println!("WINNER TAKES IT ALL!");
     println!("ATF! ");
let r1=String::from("hello");
let r2=r1.clone();
println!("r1={} ,r2={} ",r1,r2);
let len:usize=calculate_length(&r1);
println!("The valus of {} is {}",r1,len);

}
// Function Implementation
fn add(item1:u8,item2:u8)->u8{
    return item1+item2;
}

//Function Implementatiton

fn calculate_length(s2:&String)->usize{
    return s2.len();
}
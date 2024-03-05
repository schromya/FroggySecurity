/*  Example 2: Ownership

    Compile ->  rustc ex_2.rs
    Run ->      ./ex_2
*/

fn main() {
    let string_1:String = "Stay Froggy!".to_string();
    let string_2:String = string_1;  // Compiler won't let you do this

    println!("{}", string_1);
    println!("{}", string_2);  // Nor this
}
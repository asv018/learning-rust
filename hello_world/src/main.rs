/* Use `carego run` command to run the file */
fn main() {
    //rust don't care about identation
     println!("Hello, world!");
    
    //VARIABLES IN RUST
    let x=10;
   
    println!("x is {} which is x",x); //{} id important as the x will be placed here

    /*
    x=20 can not be done as cannot assign twice to immutable variable
    By default variable is rust are "immutable" ie cannot be changed
    Variable must be explicitly declared as mutable using mut keyword

    */
    let mut y=10;
    println!("y is {} at first",y);
    y=20;
    println!("y is now {}",y);

}

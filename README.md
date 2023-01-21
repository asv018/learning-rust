# learning-rust
cpdespace for learning rust

<h1>Here are my Notes on rust lang</h1>
<ul>
<li>Use `carego run` command to run the file</li>
<li>Rust don't care about identation</li>
<li> A var can be assigned by using let keyword</li>
<li>Variable defined by `let` keyword can not be reassigned as cannot assign twice to immutable variable</li>
<li> By default variable is rust are "immutable" ie cannot be changed</li>
<li>Variable must be explicitly declared as mutable using mut keyword</li>
<li>Rust is `statically typed language` i.e. All variables data types must be known at compile time like C++ , Java and Go</li>
<li>Data types in Rust: 
<ul>
<li> Integer: characterised by number of bits, can be signed and unsigned. By default int in rust are signed 32bit type
Example:
 let x:u8 =100;
  // u8 is for undigned 8 bit number if we assign a numbe greater than 255 which is max of what u8 can store, it gives error. Many programming languages do not give such errors.</li>
<li> Floating Point: Represt numbers using IEEE 754 standard. Rust has two floating types `f32` and `f64`</li>
<li>Numereic type: </li>
<li>Boolean type: </li>
<li>Compound Type: 
Tuple type:
Array type:
</li>
</ul>
</li>

<li>for casting datatypes `as` keyword is used 
e.g. 3 as f64 gives 3.0 int to floating
but in case of float to int 3.9 as i32 gives 3
<br/>Some strange results:😕 casting 300 as u8 gives 44 as u8 can store upto 256 the remainde is casted 
Casting -300 as u32 gives 4294966996(not exact) this is (upper limit of u32 - 300) </li>
<li></li>
<li></li>
<li></li>
<li></li>

</ul>
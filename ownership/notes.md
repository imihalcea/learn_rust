#Ownership

    * Each value is owned by a variable
    * When owner (the variable) goes out of scope => it will be deallocated (RAII in C++)
    * Only one owner at a time

##Example of code (working)
```rust 
fn main() {
    
    // size not known at compile time => Heap
    // mut means mutable. variable should be explicitly declared mutable
    let mut input = String::new(); 
    
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = i32::from_str(&input.trim()).unwrap();
    let next = next(n);
    println!("The next is {}", next);
}

fn next(n:i32) -> i32{
    n+1
}
```

##Example of code (ownership moved => compiler error)
```rust
fn main() {
    let mut input = String::new(); //Heap allocated
    let mut x = input; //moved here ownership of the value changed input -> x
    
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    //input is used here but it is no longer the owner of the value
    //since string is a complex type allocated on the heap 
    //a copy can't be created since it does not implement the copy trait
    //the compiler error is clear
    io::stdin().read_line(&mut input).unwrap();
    
    let n = i32::from_str(&input.trim()).unwrap();
    let next = next(n);
    println!("The next is {}", next);
}

fn next(n:i32) -> i32{
    n+1
}
```
###The compiler error
```
error[E0382]: borrow of moved value: `input`
 --> src/main.rs:9:27
  |
6 |     let mut input = String::new();
  |         --------- move occurs because `input` has type `String`, which does not implement the `Copy` trait
7 |     let mut x = input;
  |                 ----- value moved here
8 |     io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
9 |     io::stdin().read_line(&mut input).unwrap();
  |                           ^^^^^^^^^^ value borrowed here after move
```


##Example of code (ownership moved => compiler error)
```rust
fn main() {
    let mut input = String::new();
    dummy_fn(input); // the ownership moves here 
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = i32::from_str(&input.trim()).unwrap(); //boom here
    
    let next = next(n);
    println!("The next is {}", next);
}

fn dummy_fn(s:String){}
```

###The compiler error
```
error[E0382]: borrow of moved value: `input`
 --> src/main.rs:9:27
  |
6 |     let mut input = String::new();
  |         --------- move occurs because `input` has type `String`, which does not implement the `Copy` trait
7 |     dummy_fn(input);
  |              ----- value moved here
8 |     io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
9 |     io::stdin().read_line(&mut input).unwrap();
  |                           ^^^^^^^^^^ value borrowed here after move
```


#Borrowing

##Passing references as parameters

```rust
fn main() {
    let mut input = String::new();
    dummy_fn(&input); // pass a reference it doesn't change the ownership 
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = i32::from_str(&input.trim()).unwrap();
    let next = next(n);
    println!("The next is {}", next);
}

//the argument is a reference
//the function borrows the string
//which means that it wont be deallocated 
//when goes out of scope
fn dummy_fn(s:&String){} 
```

##Passing mutable references as parameters

```rust
fn main() {
    let mut input = String::new();
    dummy_fn(&mut input); // pass a reference it doesn't change the ownership 
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = i32::from_str(&input.trim()).unwrap();
    let next = next(n);
    println!("The next is {}", next);
}

//the argument is a reference
//the function borrows the string
//which means that it wont be deallocated 
//when goes out of scope
fn dummy_fn(s:&mut String){
    s.push_str("dummy");
} 
```

###Important: In a scope we can have many immutable references or only one mutable 


##Example of code (many immutable references)
```rust
fn main() {
    let mut input = String::new();
    
    let ref1 = &input;
    let ref2 = &input;
    println!("{} {}", ref1, ref2);
    
    dummy_fn(&mut input); // pass a reference it doesn't change the ownership 
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = i32::from_str(&input.trim()).unwrap();
    let next = next(n);
    println!("The next is {}", next);
}

fn dummy_fn(s:&mut String){
    s.push_str("dummy");
} 
```
##Example of code (one mutable references and one immutable)

### => Rust prevents race conditions at compile time

```rust
fn main() {
    let mut input = String::new();

    let mut ref1 = &mut input;
    let ref2 = &input;
    println!("{} {}", ref1, ref2);

    dummy_fn(&input);
    io::stdout().write_all("Enter a number : \n".as_bytes()).unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let n = i32::from_str(&input.trim()).unwrap();
    let next = next(n);
    println!("The next is {}", next);
}

fn dummy_fn(s:&String){}
```
```
error[E0502]: cannot borrow `input` as immutable because it is also borrowed as mutable
  --> src/main.rs:9:16
   |
8  |     let mut ref1 = &mut input;
   |                    ---------- mutable borrow occurs here
9  |     let ref2 = &input;
   |                ^^^^^^ immutable borrow occurs here
10 |     println!("{} {}", ref1, ref2);
   |                       ---- mutable borrow later used here
```

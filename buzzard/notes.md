##Associated functions and methods


```rust
impl Server {
    fn new(address:String, port:u16) -> Self {
        Server{address, port}
    }

    fn run(self){
        info!("Buzzard is running");
    }
}
```
Run method takes the ownership of the Server instance.
At the end of the function we want it to be deallocated.
We do not expect to use the instance of the server outside of run method.

    new : 
        - is an associated function of struct Server
        - good practice to name new the constructors
        - constructors can have the name we want
        - "Self" an alias to the structs name

    run :
        - is a method
        - keyword self (equivalent of this keyword in C#/Java)
        - &self a reference
        - &mut self mutable reference

##Strings
    String :
        - complex structure
        - kind of a smart pointer
        - heap allocation
    &str : immutable slice of string
        - similar to string view in C++
        - Span in C# ?
    string literal
        - size known at compile time
        - immutable
        - is a string slice

    All strings in rust are UTF8. 1 char is 1 to 4 bytes
    Take care about &string[..3]


##Organize code in modules
    structure
    visibility
    public API
    one file == one module
    one folder + mod.rs file == 1 module with nested modules

##Result & Error
    Error<T> recoverable error
    Panic unrecoverable => program halts

```rust
    pub fn run(self) {
    let listener = TcpListener::bind(&self.address).unwrap(); //panics if error
    info!("Listening on {}", self.address);
    loop{
        let result = listener.accept();
        match result { //pattern matching
            Ok((stream, client_address)) => {}
            Err(err) => {
                error!("{}",err);
                continue;
            }
        }
    }
}
```

    

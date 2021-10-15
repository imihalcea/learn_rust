
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


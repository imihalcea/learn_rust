
struct A{
    f:Box<dyn Fn(i16) -> i16>
}

impl A {
    fn new(f: Box<dyn Fn(i16) -> i16>)-> Self{
        Self{f}
    }

    fn apply(&self,x:i16)->i16{
        (self.f)(x)
    }
}

fn main() {
    let f1 = |x|x+1;
    let a2 = A::new(Box::new(|x|x+2));
    let a3 = A::new(Box::new(|x|x+3));
    println!("{}",f1(1));
    println!("{}",a2.apply(1));

    let funcs = vec![a2,a3];
    for func in funcs {
        println!("{}",func.apply(4));
    }


    let funcs2 = vec![Box::new(|x|x+2), Box::new(|x|x+3)];
    for func in funcs2 {
        println!("{}",func(4));
    }
}

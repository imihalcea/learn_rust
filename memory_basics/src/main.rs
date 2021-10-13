fn main() {
    let n = 2;
    let result = stack_only(n);
    dbg!(result);
}

fn stack_only(x: i32) -> i32 {
    let y = 3;
    return x + y + stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let z = 8;
    let w = Box::new(7); // allocation on heap with smart pointer
    return z + *w;
}


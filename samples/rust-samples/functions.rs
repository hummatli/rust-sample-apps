fn retSomething() -> i32 {
    return 1;
}

fn retSomething2() -> i32 {
    1
}

fn retSomething3() -> &'static str {
    return "ddd";
}

fn main() {
     println!("{}", retSomething());
     println!("{}", retSomething2());
     println!("{}", retSomething3());
}

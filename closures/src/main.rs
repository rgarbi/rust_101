fn main() {

    let func = || -> String {
        println!("works!");
        return String::from("It Did");
    };

    thing_to_do(func);
}

fn thing_to_do<F: FnOnce() -> T>(func: F) {
    let out = func();
    println!("{}", out);
}

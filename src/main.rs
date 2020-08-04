use proc_macro_derive::hello_macro_derive;

struct Pancakes;

fn call<F: Fn()>(closure: &F) {
}

#[hello_macro_derive]
impl Pancakes {
    fn test() {
        let test: i32 = String::new();
        /*let closure = || {
        };
        call(&closure);*/
        call(&|| {
        });
    }
}

fn main() {
}

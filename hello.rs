#![feature(core_intrinsics)]
fn main(){

    fn print_type_of<T>(_: &T) {
        println!("{}", unsafe { std::intrinsics::type_name::<T>() });
    }

    let a = {
        let inner = 3;
        inner * inner
    };
    let b = {
        let inner = 3;
        inner * inner;
    };

    print_type_of(&a);
    print_type_of(&b);
    // rustc hello.rs && ./hello
}


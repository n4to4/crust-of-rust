#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    // function item
    // - zero-sized value
    // - https://doc.rust-lang.org/reference/types/function-item.html
    //let x = bar;
    let x = bar::<i32>;
    println!("{}", std::mem::size_of_val(&x));

    baz(bar::<i32>);
    baz(bar::<u32>);
    quox(bar::<u32>);
}

fn bar<T>() {}

fn baz(f: fn()) {
    println!("{}", std::mem::size_of_val(&f));
}

fn quox<F>(f: F)
where
    F: Fn(),
{
}

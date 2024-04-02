use derive_macro::Display;

#[derive(Display)]
struct Foo{
    x: i32,
    y: u32,
}

fn main() {
    let v = Foo {
        x: -42,
        y: 17
    };
    println!("{}", v);
}

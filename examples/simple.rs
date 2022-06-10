use injectable::{inject, provide};

fn main() {
    let provider = provide! {
        i32 => 1,
        String => String::from("foo"),
    };

    let output = foo(&provider);

    println!("output = {output}");
}

#[inject]
fn foo(x: i32, _: String) -> String {
    format!("{x} => {}", bar(inject!()))
}

#[inject]
fn bar(y: String) -> String {
    format!("y={y:?}")
}

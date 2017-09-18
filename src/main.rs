macro_rules! pipe {
    ($e:expr) => ($e);
    ($e:expr => $f:ident) => ($f($e));
    ($e:expr => $f:ident | $($rest:ident) | *) => (
        pipe!($f($e) => $($rest) | *)
    );
}

fn main() {
    let exclaim = |s| {s + "!"};
    let capitalize = |s: String| {
        let mut c = s.chars();
        c.next().unwrap().to_uppercase().collect::<String>() + c.as_str()
    };
    let double_say = |s| String::new() + s + ", " + s;

    let s = pipe!("hello" => double_say | capitalize | exclaim);
    println!("{}", s);
}

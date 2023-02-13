#[test]
fn example1() {
    let mut v: Vec<char> = Vec::with_capacity(42);
    // drop(v);

    // ...
    v.push('c');
    println!("{}", v[0]);
}





fn g<'a,'b>(a : &'a mut i32, b : &'b mut i32) -> &'b mut i32 {
    b
}
#[test]
fn example2() {
    let mut x : i32 = 3;
    let mut y : i32 = 4;
    let p = g(&mut x, &mut y);
    *p = 42;
}


fn get_string() -> String {
    String::from("sdflksjdl")
}
#[test]
fn example3() {
    let s = get_string();
    println!("{}", s);
}



#[test]
fn example4() {
    let mut v = vec![];
    v.push(0);
    let first_elem = &v[0];
    v.push(1);
    // println!("{}", *first_elem);
}


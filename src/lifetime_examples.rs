fn test_lifetime_1() {
    let s1: &String;
    {
        let s = String::from("Hello World");
        // s1 = &s;
    }
    // println!("{}", s1);
}








fn return_largest<'a>(s1 : &'a String, s2 : &'a String) -> &'a String {
    if s1.len() > s2.len() {
        s1
    }
    else {
        s2
    }
}

// fn return_largest(s1 : &String, s2 : &String) -> &String {
//     if s1.len() > s2.len() {
//         s1
//     }
//     else {
//         s2
//     }
// }
#[test]
fn test_lifetime_2() {
    let s1 = String::from("Hello!");
    let s2 = String::from("Everybody");

    let s = return_largest(&s1, &s2);
    println!("{}", s);
}
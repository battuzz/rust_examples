fn test_lifetime_1() {
    let s1: &String;
    {
        let s = String::from("Hello World");
        // s1 = &s;
    }
    // println!("{}", s1);
}



// --------------------------------------------------------




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




// --------------------------------------------------------




// fn my_push_back<'a>(v: &mut Vec<&'a String>, s: &'a String) {
//     v.push(s);
// }

// fn my_push_back(v: &mut Vec<&String>, s: &String) {
//     v.push(s);
// }

// #[test]
// fn test_lifetime_3() {
//     let mut my_vector : Vec<&String> = Vec::new();
//     // {
//         let s = String::from("Hello!");
//         my_push_back(&mut my_vector, &s);
//     // }

//     println!("{:?}", my_vector);
// }
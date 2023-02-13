
#[test]
fn test_borrowing_1() {
    let s = String::from("Hello world");

    let s1 = &s;
    let s2 = &s;    // OK, multiple immutable borrows

    println!("{}, {}", s1, s2);
}




#[test]
fn test_borrowing_2() {
    let mut s = String::from("Hello world");

    let s1 = &mut s;  // OK, only one mut
    *s1 = String::from("Bye bye"); // Can mutate string
    
    println!("{}", s);  // Bye bye
}



#[test]
fn test_borrowing_3() {
    let mut s = String::from("Hello world");

    let s_mut = &mut s;
    let s_immut = &s;

    // println!("{}, {}", s_mut, s_immut);
}




fn print_string_2(s : &String) {
    println!("{}", s);
}
#[test]
fn test_borrowing_4() {
    let s = String::from("Hello world");
    print_string_2(&s);

    println!("{}", s);  // s is still valid
}






fn mutate_string(s : &mut String) {
    s.push_str(", World!");
}
#[test]
fn test_borrowing_5() {
    let mut s = String::from("Hello");
    mutate_string(&mut s);

    println!("{}", s);
}














#[test]
fn test_borrowing_6() {
    let mut v = vec![1, 2, 3];

    let a = &mut v[0];
    // let b = &mut v[1];

    *a = 4;
    // *b = 5;

    println!("{:?}", v);
}






#[test]
fn test_borrowing_7() {
    let mut v = vec![1, 2, 3];

    let (first, other) = v.split_at_mut(1);

    let a = &mut first[0];
    let b = &mut other[0];

    *a = 4;
    *b = 5;

    println!("{:?}", v);
}






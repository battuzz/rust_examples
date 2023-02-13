

#[test]
fn test_ownership_1() {
    let x = 5;
    let y = x;
    println!("{} {}", x, y);  // OK -> primitive values are copied
}



#[test]
fn test_ownership_2() {
    let s1 = String::from("Hello world");  // 's1' is the owner of this string
    let s2 = s1;  // Ownership is trasfered --> s2 is the new owner and s1 is dropped
    // println!("{} {}", s1, s2);  // Compiler error -> s1 is not valid anymore
}


fn print_string(s: String) {
    println!("The string is: {}", s);
}
#[test]
fn test_ownership_3() {
    let s = String::from("Hello world");

    print_string(s);    // MOVE!!
    // s not valid anymore
    // println!("{}", s);
}


fn print_string_return(s: String) -> String {
    println!("The string is: {}", s);
    s
}
#[test]
fn test_ownership_4() {
    let mut s = String::from("Hello world");

    s = print_string_return(s);    // Move and take backs
    println!("{}", s);
}






#[test]
fn test_ownership_5() {
    let s = String::from("Hello world");

    drop(s);
}







#[derive(Copy, Clone, Debug)]
struct MyStruct {
    val : i32
}

#[test]
fn test_copy_trait() {
    let s = MyStruct {val : 4};
    
    let s2 = s;

    println!("{s:?}, {s2:?}");
}









fn repeat_twice(s: String) -> String {
    s.repeat(2)
}

#[test]
fn test_ownership_6() {
    let mut v = vec![String::from("A"), String::from("B")];

    // repeat_twice(v[0]);


    // v[0] = repeat_twice(v[0]);



    // let tmp = v.remove(0);
    // v.insert(0, repeat_twice(tmp));



    // With option
    // let mut v = vec![Some(String::from("A")), Some(String::from("B"))];
    // v[0] = Some(repeat_twice(v[0].take().unwrap()));
}






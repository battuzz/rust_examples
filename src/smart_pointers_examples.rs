use std::{rc::Rc, cell::RefCell};


#[derive(Debug)]
struct MyStruct {
    x : i32
}
#[test]
fn smart_pointers_1() {
    let s = MyStruct {x : 10};
    let a = Box::new(s);    // Now a points to somewhere in the heap
    println!("{:?}", a);
}






#[test]
fn smart_pointers_2() {
    let s = MyStruct {x : 10 };
    let rc = Rc::new(s);

    let s1 = rc.clone();  // Increments reference count
    let s2 = rc.clone();  // Increments reference count

    println!("{:?}, {:?}", s1, s2);
}







#[test]
fn smart_pointers_3() {
    let x = RefCell::new(MyStruct {x : 10});

    // Returns &MyStruct. Panics if value is currently mutably borrowed
    let x_ref = x.borrow();

    // Return &mut MyStruct. Panics if value is currently immutably borrowed
    // let x_ref_mut = x.borrow_mut();
}




#[test]
fn smart_pointers_4() {
    let s = MyStruct {x : 10};

    let ref_s = RefCell::new(s);  // Note: IMMUTABLE

    let mut mut_ref = ref_s.borrow_mut();  // Can borrow mut from immutable object: Internal mutabily
    mut_ref.x = 15;
    drop(mut_ref);

    let immut_ref = ref_s.borrow();
    println!("{:?}", immut_ref);
}






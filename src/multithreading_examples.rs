use std::cell::RefCell;
use std::rc::Rc;
use std::thread;
use std::sync::{Arc, Mutex};

struct Counter {
    counts : i32
}
impl Counter {
    fn add_one(&mut self) {
        self.counts += 1;
    }
}


#[test]
fn multithreaded_counter_1() {
    let mut counter = Counter {counts: 0};

    for i in 0..10 {
        counter.add_one();
    }
    println!("Counts: {}", counter.counts);
}


// #[test]
// fn multithreaded_counter_2() {
//     let mut counter = Counter {counts: 0};

//     let handle = thread::spawn(|| {
//         counter.add_one();
//     });

//     handle.join().unwrap();
//     println!("{}", counter.counts);
// }




// #[test]
// fn multithreaded_counter_3() {
//     let mut counter = RefCell::new(Counter {counts: 0});

//     let r = &counter;
//     let handle = thread::spawn(move || {
//         r.borrow_mut().add_one();
//     });

//     handle.join().unwrap();
//     println!("{}", counter.borrow().counts);
// }






// #[test]
// fn multithreaded_counter_4() {
//     let counter = Counter {counts: 0};
//     let shareable_counter = Mutex::new(counter);

//     let handle = thread::spawn(move || {
//         shareable_counter.lock().unwrap().add_one();
//     });

//     handle.join().unwrap()
//     let x = shareable_counter.lock().unwrap();
//     println!("{}", x.counts);
// }






// #[test]
// fn multithreaded_counter_5() {
//     let counter = Counter {counts: 0};
//     let shareable_counter = Rc::new( Mutex::new(counter) );

//     let shared_counter = shareable_counter.clone();

//     let handle = thread::spawn(move || {
//         shared_counter.lock().unwrap().add_one();
//     });

//     handle.join().unwrap()
//     let x = shareable_counter.lock().unwrap();
//     println!("{}", counter.counts);
// }





#[test]
fn multithreaded_counter_6() {
    let counter = Counter {counts: 0};
    let shareable_counter = Arc::new( Mutex::new(counter) );

    let shared_counter = shareable_counter.clone();

    let handle = thread::spawn(move || {
        shared_counter.lock().unwrap().add_one();
    });

    handle.join().unwrap();
    let x = shareable_counter.lock().unwrap();
    println!("{}", x.counts);
}










#[test]
fn multithreaded_counter_7() {
    let n_threads = 10;
    let shareable_counter = Arc::new(Mutex::new(Counter {counts : 0}));
    
    let mut handles = vec![];
    
    for t_id in 0..n_threads {
        let counter_clone = shareable_counter.clone();
        let handle = thread::spawn(move || 
            for i in 0..1000 {
                let mut counter = counter_clone.lock().unwrap();
                counter.add_one();
            }
        );

        handles.push(handle);
    }

    for i in 0..n_threads {
        handles.pop().unwrap().join().unwrap();
    }

    println!("{}", shareable_counter.lock().unwrap().counts);
}
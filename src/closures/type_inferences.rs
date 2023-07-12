use std::thread;
use std::time::Duration;

pub fn explain() {
    println!("There are more differences between closures and functions: closures dont require type annotations");
    //you can add them tho

    let closure = |num: u32| -> u32 {
        println!("slow calc");
        thread::sleep(Duration::from_secs(2));
        num + 1
    };

    let closure_unannotated = |num| num + 1;

    //look, it has its type inferred
    closure_unannotated(5);

    //infer locked
    let infer_locked = |num| num;

    //type locked as &str
    infer_locked("hello, world");
    
    //now can't use closure as anything except &str (no i32)
    // infer_locked(16);

    //Rust infers for closures because closures are more private and specific: closures won't be exposed to end user
    //Rust COULD have made functions infer types, but that may cause confusion: 
        //like should this function accept a String or a &str, a u32 or  u64
}
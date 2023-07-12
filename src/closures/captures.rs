use std::thread;

pub fn explain() {
    let mut list = vec![1, 2, 3];

    //This closure captures as a borrow
    let mut capture_borrow = || list.push(7);

    capture_borrow();

    //So it can be printed afterwards
    println!("{list:?}");

    //If instead you want the closure to take ownership, use move

    //We make a new thread, pass the closure, the thread now owns it
    //If the explain function we're in dropped list before this thread was done with it, /
    //then a borrow would point to invalid data, therefore we HAVE TO move list to this new thread

    thread::spawn(move || println!("This owns {list:?} now")).join().unwrap();

    //To control what happens to captured values in a closure, use traits
    //FnOnce: can be called once (all closures implement this)
        //A closure that just moves captured values out of its body will only implement FnOnce, 
        //because it can only be called once
    //FnMut: can't move captures out of body, but may mutate captured values
    //Fn: don't move captured values out of body, don't mutate captured values, or don't capture anything


    //The unwrap_or_else method on Option<T> accepts a closure of type FnOnce() -> T 
        //must be able to be called no more than once, (but may not be called at all) take no arguments, and return a T

    //Functions can implement Fn types, and can be subbed out for a closure:
    let inventory_item_at_slot_15: Option<String> = None;
    let get_item = inventory_item_at_slot_15.unwrap_or_else(String::new);


    //the sort_by_key method on slices uses FnMut instead of FnOnce
    let mut list = [
        String::from("Hello"),
        String::from("Hi"),
        String::from("Howdy"),
        String::from("Yo what's good"),
        String::from("Heya"),
    ];

    //Can't use FnOnce: sort_by_key calls the closure once for each item in the list
    list.sort_by_key(|val| val.len());
    println!("{list:#?}");

    //We can't use a closure that moves a value out of the closure, because that only implements FnOnce
    let mut confused: Vec<String> = vec![];

    //apparently this tries to count the number of times sort_by_key's closure gets called during the sort
    //that moves ownership to confused, now sort_by_key can't use val and show's over!
    //Aaaaand then it can never be called again because now value isn't in the closure, so this implements FnOnce and that's it
    let mut counter = 0;
    list.sort_by_key(|val| {
        // confused.push(val);
        //a better way to count how many times the closure is called is to have an internal counter
        counter += 1;
        val.len()
    });
    println!("The closure was called {counter} times");
}
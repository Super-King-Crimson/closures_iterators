use std::slice::Iter;

pub mod r#trait;
pub mod methods;
pub mod iterator_adaptors;
pub mod env_captures;

pub fn explain() {
    println!("An iterator performs a task on a sequence of items.");
    //IN rust, iterators are lazy: they do nothing until methods are called to use the iterator

    let vec: Vec<i32> = vec![1, 2, 3];
    let vec_iter: Iter<'_, i32> = vec.iter();

    //We can use this iterator in many ways, like a for loop:
    for item in vec_iter {
        println!("{item}");
    }
    //This is what a for loop does under the hood when you call it with a vector: make iterator, loop over it
    r#trait::explain();
}

//Yo quick trivia: there are 3 types of ways to make an iterator from a collection:
    //iter (that's what we usually use) iterates over references (&T)
    //iter_mut iterates over mutable references (&mut T)
    //into_iter iterators over the values in the collection itself (T)

fn back_to_main() {
    //The only method that an iterator is REQUIRED to implement is next
    println!("Next returns Some with a value until the iterator gets to the end of its list, then it returns None");
    
    let vec = vec![1, 2, 3];
    let mut vec_iter = vec.iter();

    println!("First value is {:?}", vec_iter.next());
    println!("Second value is a reference to 2, correct? {}", &2 == vec_iter.next().unwrap());
    println!("Third value is {:?}", vec_iter.next());
    println!("No fourth value, so it's {:?}", vec_iter.next());
    //The iterator has to be mutable because next consumes the iterator (changes its internal state)
    //A for loop makes the iterator mutable behind the scences
    methods::explain();
}

fn back_to_main2() {
    iterator_adaptors::explain();
}

fn back_to_main3() {
    env_captures::explain();
}
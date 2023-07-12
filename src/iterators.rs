use std::slice::Iter;

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
}
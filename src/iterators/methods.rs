pub fn explain() {
    println!("While next is the only required method, there are several other methods that consume an iterator");
    //Check out the docs to see them by the way

    println!("The reason why this is important is because other default implementations for methods can call next");
    //These methods are called consuming adaptors: calling them uses up the iterator

    //Let's look at one example, fold (screw you rust book sum is lame)
    let vec = vec!['a', 'b', 'c', 'd', 'e'];
    let vec_iter = vec.iter();

    let mut init = String::new();
    let folded = vec_iter.fold(init, |init, x| format!("{init}{x}"));
    println!("{folded}");

    super::back_to_main2();
}
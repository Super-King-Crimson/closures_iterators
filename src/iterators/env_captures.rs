pub fn explain() {
    println!("\
Many iterator adapters take a closure for an argument, 
and many of these closures will take something from their environment");

    //You know the drill, here's filter()
    let vec = vec!["Hello ".to_string(), "World!".to_string(), "!!!!!".to_string()];
    let vec_iter = vec.iter();

    let search_for = '!';
    //This immutably captures search_for
    let contains: Vec<_> = vec_iter.filter(|s| s.contains(search_for)).collect();

    println!("All characters in this list\n-----------\n{:#?}\n-----------", contains);
    println!("contain '{search_for}'");
}
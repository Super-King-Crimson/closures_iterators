//You'll understand why this is here very soon in a bit
#[warn(unused)] 
pub fn explain() {
    println!("\
Iterator adaptors don't consume the adaptor, 
instead they produce different iterators by changing some aspect of the original iterator");

    //Let's look at an example, map
    let vec = vec![3, 2, 1, 0];

    //Remember? Iterators are lazy. map() produces an iterator that calls the provided closure on each element,
    //But it doesn't run this closure by itself, WE have to
    let mapped = vec.iter().map(|x| x + 1);

    //Once we consume it with a consuming adaptor, clippy stops yelling at us
    let vec2: Vec<_> = mapped.collect();
    println!("{:?}", vec2); //[4, 3, 2, 1]

    //Remember to consume your iterators kids! (?)
    super::back_to_main3();
}
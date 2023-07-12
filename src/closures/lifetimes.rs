pub fn explain() {
    //pretend the flawed version of make_a_cloner was allowed to compile:
    let hi = String::from("hi guys");
    // let cloner = make_a_cloner(&hi);
    
    //alright hi's not used ever again let's drop it
    // drop(hi);

    //uh oh now what??? invalid memory pointer inside that closure's environment
    //we have to tell rust that the str it gets to make that closure has to live longer than the closure itself
    // let str2 = cloner();    

    //removes O perms from hi
    let cloner = make_a_cloner_lifetimes(&hi);
    
    //now that rust knows its safe, it will prevent us from being goofy
    // drop(hi);

    println!("{}", cloner());
}

//this is supposed to make something that clones a str slice
fn make_a_cloner(s_ref: &str) /* -> impl Fn() -> String */ {
    //according to the error, 'hidden type captures an lifetime that doesn't appear in bounds'
    //captures s_ref from make_a_cloner's args
    // move || s_ref.to_string()
}

//this + 'a is for the closure, not the string
//the closure itself can't live longer than 'a
//and now it's safe!
fn make_a_cloner_lifetimes<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

//or we could use elison rules: all the function needs to know is that the returned value depends on some lifetime
fn concise_make_a_cloner_lifetimes(s_ref: &str) -> impl Fn() -> String + '_ {
    move || s_ref.to_string()
}
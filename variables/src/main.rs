fn main() {
    let mut s = String::from("hello");
    println!("before the call, {}", s);

    {
        let r1 = &mut s;
        change(r1);
        println!("after the call, {}", r1);
    }

    {   
        println!("between the calls, {}", s);
        let mr1 = &s; // immutable reference
        println!("between the calls, imm. ref.1 {}", mr1);
        let mr2 = &s; // immutable reference
        println!("between the calls, imm. ref.2 {}", mr2);

    }

    {
        let r2 = &mut s;
        change(r2);
        println!("after the second call, {}", r2);
    }

    println!("after all calls, {}", s);
}

fn change(some_string: &mut String) {
    println!("in the function call, before modify: {}", some_string);
    some_string.push_str(", world");
    println!("in the function call after modify: {}", some_string);
}
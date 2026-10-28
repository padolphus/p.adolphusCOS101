fn main() {
    let v = vec![10, 20, 30];
    // vector v owns the object in heap

    let v2 = &v; // borrow v instead of moving ownership

    display(v2);

    println!("In main {:?}", v2);
    // v2 is still usable here
}

fn display(v: &Vec<i32>) {
    println!("inside display {:?}", v);
}
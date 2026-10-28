fn main() {
    let v = vec![101, 250, 330, 400];
    // vector v owns the object in heap

    // only a single variable owns the heap memory at any given time
    let v2 = v.clone();
    // clone creates a copy of the heap data

    // now both variables can be used safely
    println!("{:?}", v);
    println!("{:?}", v2);
}
pub fn tuples_arrays(){
    // Arrays [Type;Size]
    // Elements same type
    let mut a: [i8; 10] = [1; 10];
    let b = [1,3,5,7,9];
    a[5] = 0;
    //b[1] = 1;  // Error, no es mutable

    println!("a: {a:#?}");
    println!("b: {b:?}");

    // Tuples
    // Elements different types
    let t: (i8, bool) = (2, true);
    println!("t.0: {}", t.0);
    println!("t.1: {}", t.1);
}
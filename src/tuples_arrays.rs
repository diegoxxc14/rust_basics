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

#[rustfmt::skip]
pub fn patrones(input: char){
    // Las comparaciones se hacen de arriba a abajo y gana el primer match.
    // También puede ser como expresión
    match input {
        'q'                         => println!("Quitting"),
        'a' | 's' | 'w' | 'd'       => println!("Moving around"),  // | or
        '0'..='9'                   => println!("Number input"),
        key if key.is_lowercase() => println!("Lowercase: {key}"),
        _                           => println!("Something else"),  // comodín para cualquier valor (Caso final)
    }
}
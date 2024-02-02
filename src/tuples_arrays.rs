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

// Una tupla de enteros como parámetro
pub fn describe_point(point: (i32, i32)){
    match point {
        (0, _)      => println!("eje Y"),
        (_, 0)      => println!("eje X"),
        (x, _) if x < 0 => println!("izquierda del eje Y"),  // 'x' como parámetro para usarlo
        (_, y) if y < 0 => println!("debajo del eje X"),
        _           => println!("1er cuadrante"),
    }
}

pub fn triple_array(triple: [i8; 3]){
    println!("Tell me about {triple:?}");
    match triple {
        [0, y, z]   => println!("First is 0, y={y} and z={z}"),
        [1, ..]             => println!("First is 1 and the rest were ignored"),
        _                   => println!("All elements were ignored")
    }
}
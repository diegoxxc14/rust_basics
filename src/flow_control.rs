pub fn conditionals(){
    let x = 10;
    if x < 20 {
        println!("small");
    } else if x < 100 {
        println!("biggish");
    } else {
        println!("huge");
    }

    // if como expresión
    let v = 10;
    // Como expresión debe terminar con ;
    let size = if v < 20 { "small" } else { "large" };
    println!("number size: {}", size);
}

pub fn bucles(){
    let tabla = 4;
    let mut index = 1;
    println!("--> BUCLE while <--");
    while index <= 12 {
        let res = index * tabla; // Ojo 'res' no es mutable, ya que el valor no cambia luego de declararla
        println!("{tabla} x {index} = {res}");
        index+=1;
    }

    println!("--> BUCLE for <--");
    for i in 1..=12 {  // No incluye el último valor, agregar = para incluirlo
        let res = i * tabla;
        println!("{tabla} x {i} = {res}");
    }

    println!("--> BUCLE loop <--");
    let mut i2 = 1;
    loop {
        let res = i2 * tabla;
        println!("{tabla} x {i2} = {res}");
        if i2 == 12 {
            break;
        }
        i2 += 1;
    }
}

pub fn bucles2(){
    // Declaración multiple
    let (mut a, mut b) = (100, 52);
    // loop como expresión
    let res = loop {
        if a == b {
            break a; // Detiene el bucle y devuelve a
        }
        if a < b {
            b -= a;
        } else {
            a -= b;
        }
    };

    print!("{res}");
}

pub fn bucles3(){
    'outer: for x in 1..5 {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;  // Se detiene el bucle exterior tras tres iteraciones del bucle interno.
            }
        }
    }
}

pub fn bloques(){
    let z = 13;
    let x = {
        // Si la última expresión del bloque termina con ;, el tipo y el valor resultante será ()
        let y = 10;
        println!("y: {y}");
        // Sino el tipo y el valor será:
        z - y
    };

    println!("x: {x}");
}

pub fn ambitos_var(){
    let a = 10;
    println!("before: {a}");
    {
        let a = "hello";
        println!("innner scope: {a}");

        // Después del sombreado las ubicaciones de memoria de ambas variables existen al mismo tiempo
        let a = true;
        println!("shadowed in inner scope: {a}");

        //let b = 10;
    }

    println!("after: {a}");
    //println!("b: {b}");  // Error - no access, only inner block

}
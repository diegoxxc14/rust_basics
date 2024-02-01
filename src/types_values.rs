pub fn variables() {
    // Las variables por defecto son inmutables. Usar mut para hacerlas mutables
    let mut x: i32 = 20;
    println!("x1: {x}");
    x=200;
    println!("x2: {x}");

    // VARIABLES (Siempre deben estar inicializadas)
    let estado: bool = true;
    let car: char = '🚗';
    let edad: u8 = 36;  // u8=0..255, u16=0..65535, u32=0..4294967296, u64, u128, usize
    let num: i32 = -2333;  // similar al anterior pero con i
    let saldo: f32 = 454.56;  // f64

    println!("bool: {}", estado);
    println!("char: {}", car);
    println!("unsigned int: {}", edad);
    println!("signed int: {}", num);
    println!("float: {}", saldo);
}

pub fn interproduct(a:i32, b:i32, c:i32) -> i32 {
    return a * b + b * c + c * a;
}

pub fn strings_demo() {
    let greeting: &str = "Greetings";  // &str read-only
    let planet: &str = "🪐";
    let mut sentence = String::new();
    sentence.push_str(greeting);
    sentence.push_str(", ");
    sentence.push_str(planet);

    println!("Final sentence: {}", sentence);
    println!("{:?}", &sentence[0..5]);
    println!("{:?}", &sentence[11..15]); // Los bytes que corresponden al emoticon 🪐

    println!(r#"<a href="link.html">link</a>"#);  // r#""# Cadenas sin formato
    println!("<a href=\"link.html\">link</a>");
}

fn takes_u32(x: u32) {
    println!("u32: {x}")
}

fn takes_i8(y: i8) {
    println!("i8: {y}")
}

pub fn types_inference(){
    // INFERENCIA DE TIPOS
    /* No se declaran como variables de "cualquier tipo", 
    sino que en base a la INICIALIZACIÓN y a USO se asigna un tipo por defecto */
    let state = true;
    let name = "Diego Cuenca";
    let valor = 23.32;
    let x = 10; 
    let y = 20;
    takes_u32(x);
    takes_i8(y);
}

// For n>2, the n’th Fibonacci number is calculated recursively as the sum of the n-1’th and n-2’th Fibonacci numbers.
pub fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

// Test function
pub fn message(message:&str) {
    println!("Mensaje: {message}");
}
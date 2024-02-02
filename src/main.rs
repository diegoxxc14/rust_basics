
mod types_values;
mod flow_control;
mod tuples_arrays;

fn main() {
    println!("Hello 🌐!");

    // VARIABLES
    //types_values::variables();

    // ARITHMETIC
    //println!("res: {}", types_values::interproduct(120, 100, 248));

    // STRINGS
    //types_values::strings_demo();

    // INFERENCIA DE TIPOS
    //types_values::types_inference();

    // EXERCISE 1
    //let n = 4;
    //println!("fib(n) = {}", types_values::fib(n));

    // CONTROL DE FLUJOS
    //flow_control::conditionals();
    //flow_control::bucles();
    //flow_control::bucles2();
    //flow_control::bucles3();
    //flow_control::bloques();
    //flow_control::ambitos_var();

    // FUNCTION
    //println!("Return: {}", types_values::message("Hola mundo"));

    // MACROS
    //let n = 4;
    //println!("{n}! = {}", flow_control::factorial(4));

    // EXERCISE 2
    //println!("Length: {}", flow_control::collatz_length(11));

    // TUPLES AND ARRAYS
    //tuples_arrays::tuples_arrays();

    // PATTERNS
    tuples_arrays::patrones('d');
    // Desestructurar Enums
    //  Tuple
    tuples_arrays::describe_point((0,-1));
    //  Array
    tuples_arrays::triple_array([5,-2,3]);
}

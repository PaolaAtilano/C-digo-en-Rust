fn main()
{
    //declaración de variables
    let x = 5;
    //ambiente para shadowing de variable x
    {    let x = x+1;
        println!("El valor de x es: {}", x);
    }
    println!("El valor de x es: {}", x);

    //declaración de variables mutables
    let mut y = 10;
    y = 20;

    //impresión de mensaje
    println!("Hello world");

    //declaración de tipo de datos
    let entero: i32 = 42;
    let flotante: f64 = 3.1416;
    let booleano: bool = true;
    let caracter: char = 'a';
    
    //declaración de estructura de datos
    let tupla: (i32, f64, char) = (43, 3.1516, 'b');
    let arreglo: [i32; 3] = [1, 2, 3];
    println!("Tupla(tupla) forma 1: {:?}", tupla);
    println!("Tupla(tupla) forma 2: ({},{},{})", tupla.0, tupla.1, tupla.2);


}
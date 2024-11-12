use minigrep::Config;
use std::env;
fn main() {
    // Print all command line arguments
    //primero obbetenemos un iterator y lo transforma en collection
    //tenemos que decir en que tipo de dato lo transformamos
    let args: Vec<String> = env::args().collect();

    //primera variable
    //let filename = args[1].clone(); //lo clonamos para tener la propiedad del nuevo objeto
    //let query = args[2].clone();

    //creamos un objeto de la estructura Config con los valores obtenidos de los argumentos
    let config = Config::new(&args);

    println!("Archivo: {}", config.filename);
    println!("Buscar: {}", config.query);

    minigrep::run(config);
}

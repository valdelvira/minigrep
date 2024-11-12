use std::fs;

//Va la logica
pub struct Config {
    pub filename: String,
    pub query: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Debe proporcionar un nombre de archivo y una consulta");
        }

        let filename = args[1].clone();
        let query = args[2].clone();
        Config { filename, query }
    }
}

//la hacemos accesible con pub
pub fn run(config: Config) {
    //leemos el fichero. Devuelve un Result, que es como un Option
    let content = fs::read_to_string(config.filename).expect("No se pudo leer el archivo");

    //imprimo los 10 primeros caracteres
    //println!("{}", &content[..10]);

    //la función search recibe un query y un contenido
    //el & a los strings indica que estamos pasando referencias a los valores, no los valores en si.
    //Esto es necesario para que la función search pueda usar los valores de los strings sin tener que hacer una copia.
    //Además, el lifetime de la variable es el que indica que la función search puede usar
    //los valores de los strings sin tener que dejar de usarlos.
    //Esto se hace para evitar el problema de la memoria detrás del código.
    let found = search(&config.query, &content);

    for line in found {
        println!("{}", line);
    }
}

//hay que añadir el lifetime de la variable
fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    //buscamos la cadena en el contenido
    let mut result = Vec::new();
    //iteramos sobre las líneas del contenido y si contiene la cadena la añadimos al vector
    for line in content.lines() {
        if line.contains(query) {
            //la añadimos al vector
            result.push(line);
            //println!("{}", line);
        }
    }
    result
}

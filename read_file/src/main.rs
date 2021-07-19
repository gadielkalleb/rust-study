use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;

fn main() {
    // let path = Path::new("/home/kalleb/projetos/rust/read_file/src/hello.txt");
    //é preciso que o arquivo esteja no diretorio padrão do rust
    // nesse caso na raiz se vc colocar o arquivo dentro de src ele não vai funcionar
    let path = Path::new("hello.txt");
    let display = path.display();

    // lembra um callback do node
    let mut file = match File::open(&path) {
        Err(err) => panic!("Couldn't open: {} --- {}", err.to_string(), display),
        Ok(value) => value,
    };

    let stat = match file.metadata() {
        Err(err) => panic!("Couldn't get stat: {}", err.to_string()),
        Ok(value) => value,
    };

    let mut buffer = vec![0; stat.len() as usize];

    match file.read(&mut buffer) {
        Err(err) => panic!("Couldn't read: {}", err.to_string()),
        Ok(_) => (),
    };

    let data = match from_utf8(&buffer) {
        Err(err) => panic!("Couldn't convert buffer to string: {}", err.to_string()),
        Ok(value) => value,
    };

    println!("Content is: {} ", data);
    println!("display {}", display);
    println!("stat : {}", stat.len());
}

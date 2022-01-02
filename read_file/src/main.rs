use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::from_utf8;
// use std::io::prelude::*;

fn read_file1() -> std::io::Result<String> {
  // let path = Path::new("/home/kalleb/projetos/rust/read_file/src/hello.txt");
  // é preciso que o arquivo esteja no diretorio padrão do rust
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

  Ok(std::string::String::from(data).to_string())
}

fn read_file2() -> std::io::Result<String> {
  let mut file = File::open("hello.txt")?;

  let metadata = file.metadata()?;

  let mut buffer = vec![0; metadata.len() as usize];

  // para fazer a leitura do arquivo preciso usar o import
  // use std::io::Read;
  file.read(&mut buffer)?;

  let data = from_utf8(&mut buffer).unwrap();

  Ok(std::string::String::from(data).to_string())
}

fn main() {
  let file1 = read_file1();
  let file2 = read_file2();
  println!("read_file2: {:?}", file2);
  println!("read_file1: {:?}", file1);
}

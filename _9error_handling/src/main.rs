use std::fs::File;
use std::io::ErrorKind;

fn main() {
  // ini adalah contoh menggunakan error handling
  // jika terdapat error kind not found maka akan mengcreate hello.txt jika tidak bisa create akan 
  // keluar problem
  // this is one of best practive
  let _file = File::open("hello.txt").unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create("hello.txt").unwrap_or_else(|error| {
        panic!("Problem creating the file: {:?}", error);
      })
    } else {
      panic!("Problem opening the file: {:?}", error)
    }
  });

}

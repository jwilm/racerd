extern crate iron;
extern crate router;
extern crate bodyparser;
extern crate racer;
extern crate crypto;

#[derive(Debug)]
pub struct Config {
    pub port: usize,
    pub secret_file: String
}

#[allow(unused_variables)]
pub fn serve(config: Config) {
    println!("{:?}", config);
    unimplemented!();
}

#[test]
fn it_works() {
}

use std::{error::Error, fs};

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let errstr = String::from("error");
    if config.query.eq(&errstr) && config.filename.eq(&errstr) {
        print!("not enough arguments");
    } else {
        let contents = fs::read_to_string(config.filename)?;
        println!("with text:\n{}", contents);
    }
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        args.next();

        let query = match args.next() {
            Some(arg) =>arg,
            None => return Err("Did't get query string"),
        };
        let filename = match args.next() {
            Some(arg) =>arg,
            None => return Err("Did't get filename string"),
        };
        
        Ok(Config { query, filename })
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
#[test]
fn using_other_iterator_trait_methods() {
    let conte: [i64;12];
    let tmpitra = Counter::new().zip(Counter::new().skip(1));
    let sum: u32 = tmpitra.map(|(a, b)| a * b).filter(|x| x % 3 == 0).sum();
    assert_eq!(18,sum)
}

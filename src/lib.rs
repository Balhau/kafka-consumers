extern crate regex;
extern crate serde;
extern crate rand;
#[macro_use]
extern crate serde_derive;

pub mod kafka;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

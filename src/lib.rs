#![no_std]

pub extern crate lpc11uxx;
extern crate embedded_hal as hal;
extern crate cortex_m;

pub mod delay;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

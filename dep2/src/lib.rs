#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

#[no_mangle]
pub extern "C" fn foo(x: u8) {
  println!("{}", x);
}

fn hello() {
    println!("Hello world!")
}

#[cfg(test)]
mod tests {
    use crate::hello;

    #[test]
    fn test_hello() {
        hello()
    }
}
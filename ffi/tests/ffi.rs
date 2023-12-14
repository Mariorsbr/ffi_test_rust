extern "C" {
    pub fn soma(a: i32, b: i32) -> i32;
}


#[cfg(test)]
mod tests {
    use crate::soma;

    #[test]
    fn test_soma(){
        assert_eq!(unsafe {soma(1,1)}, 2);
    }

}

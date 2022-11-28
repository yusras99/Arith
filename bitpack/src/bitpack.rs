use std::convert::TryInto;
use std::ops;


/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool{
    //if number is negative then see if it is greater 
    //than 1 shifted left by width -1 (bcoz preserve msb)
    if n<0{
        n >= -((1 as i64) << (width-1))
    }
    else{
        n < (1<<(width))
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {
    if n <= (1<<width){
        return true;
    }
    else {
        return false;
    }
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
    if width == 0{
        word as i64
    }
    //if word is bigger or equal to the max of value
    else{
        if word > word << (64-(lsb+width)+1){
            (word<<(64-(width+lsb)+1)) as i64 >> (64-width)

        }
        else {
            (word << (64-(lsb+width))) as i64 >> (64-width)
        }
    }
    
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u64, width: u64, lsb: u64) -> u64 {
    if width == 0{
        return word;
    }
    let left = 64 - (lsb+width);
    let right = 64 - width;
    //word>>lsb
    //shift left your word by start
    //and then shift it right by start
    //interpret that
    let mut new_word  = word << left;
    new_word  = new_word >> right;
    return  new_word;
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    //Some(0)
    if fitsu(value, width){
        let packed = word | (value << lsb);
        Some(
            packed
        )
    }
    else {
        None
    }

}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    //Some(0)
    if fitss(value, width){
        let hi = ((word >> (lsb+width))<<(lsb+width)) as i64;
        let lo = ((word<<(64-lsb))>>(64-lsb)) as i64;
        let field = value<<lsb;
        let new_word = lo | hi | field;
        Some(new_word.try_into().unwrap())
    }
    else {
        None
    }
}

    fn check_laws(word: u64, width: u64, lsb: u64, value: u64,width2:u64, lsb2:u64){
        if fitsu(value, width){
            assert!(getu(newu(word, width, lsb, value).unwrap(),width,lsb) == value);
        }
    }

#[cfg(test)]
mod tests {

    use crate::bitpack::{fitss,fitsu, getu, gets, newu,news};
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
    #[test]
    fn fitss_test(){
        assert!(fitss(-8,4));
        assert!(!(fitss(-8,3)));
        assert!(fitss(5,3));
    }
    #[test]
    fn fitsu_test(){
        assert!(fitsu(7 , 4));
        assert!(fitsu(5,3));
    }
    #[test]
    fn getu_test(){
        assert_eq!(getu(0x3f4, 6, 2),61);
    }
    #[test]
    fn gets_test(){
        assert_eq!(gets(0x3f4, 6, 2), -3);
    }
    #[test]
    fn news_test(){
        assert_eq!(news(0_u64,9, 23,10).unwrap(), 83886080);
    }
    /*#[test]
    fn check_laws(word,w,lsb,value){
        if fitsu(value, w){
            assert_eq!(getu(newu(word, w, lsb, value).unwrap(),w,lsb) == value);
        }
    }*/
}
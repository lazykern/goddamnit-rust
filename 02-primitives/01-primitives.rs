fn main() {

    // IMMUTABLE

    let logical: bool = true;

    // regular annotation
    let a_float: f64 = 1.0;
    // suffix annotation
    let an_integer = 5i32;


    // MUTABLE

    let mut an_mut_integer = 12;
    an_mut_integer = 1287361872i64;

    let mut mutable = 12;
    mutable = 21;

    // Error: the type cannot be changed
    // mutable = true
    
    // Variable can be overwritten with shadowing
    let mutable = true;


}

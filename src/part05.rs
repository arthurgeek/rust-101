// Rust-101, Part 05: Clone
// ========================

// ## Big Numbers

#[derive(Clone)]
pub struct BigInt {
    pub data: Vec<u64>, // least significant digit first, no trailing zeros
}

// Now that we fixed the data representation, we can start implementing methods on it.
impl BigInt {
    pub fn new(x: u64) -> Self {
        if x == 0 {
            BigInt { data: vec![] }
        } else {
            BigInt { data: vec![x] }
        }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() == 0 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    // We can convert any little-endian vector of digits (i.e., least-significant digit first) into a number,
    // by removing trailing zeros. The `mut` declaration for `v` here is just like the one in `let mut ...`:
    // We completely own `v`, but Rust still asks us to make our intention of modifying it explicit. This
    // `mut` is *not* part of the type of `from_vec` - the caller has to give up ownership of `v` anyway, so
    // they don't care anymore what you do to it.
    // 
    // **Exercise 05.1**: Implement this function.
    // 
    // *Hint*: You can use `pop` to remove the last element of a vector.
    pub fn from_vec(mut v: Vec<u64>) -> Self {
        // remove trailing zeroes
        while v.len() > 0 && v[v.len()-1] == 0 {
            v.pop();
        }

        BigInt { data: v }
    }
}

// ## Cloning
fn clone_demo() {
    let v = vec![0,1 << 16];
    let b1 = BigInt::from_vec((&v).clone());
    let b2 = BigInt::from_vec(v.clone());
    let b3 = BigInt::from_vec(v);
}

//impl Clone for BigInt {
    //fn clone(&self) -> Self {
        //BigInt { data: self.data.clone() }
    //}
//}

// We can also make the type `SomethingOrNothing<T>` implement `Clone`. 
use part02::{SomethingOrNothing,Something,Nothing};
impl<T: Clone> Clone for SomethingOrNothing<T> {
    fn clone(&self) -> Self {
        match *self {
            Nothing => Nothing,
            Something(ref v) => Something(v.clone()),
        }
    }
}

// **Exercise 05.2**: Write some more functions on `BigInt`. What about a function that returns the number of
// digits? The number of non-zero digits? The smallest/largest digit? Of course, these should all take `self` as a shared reference (i.e., in borrowed form).

impl BigInt {
    pub fn number_of_digits(&self) -> usize {
        self.data.len()
    }

    pub fn number_of_non_zero_digits(&self) -> u64 {
        let mut sum = 0;

        for digit in self.data.iter() {
            if *digit != 0 {
                sum += 1
            }
        }

        sum
    }

    pub fn smallest_digit(&self) -> u64 {
        let mut smallest = self.data[0];

        for digit in self.data.iter() {
            if *digit < smallest {
                smallest = *digit
            }
        }

        smallest
    }

    pub fn biggest_digit(&self) -> u64 {
        let mut biggest = self.data[0];

        for digit in self.data.iter() {
            if *digit > biggest {
                biggest = *digit
            }
        }

        biggest
    }
}

pub fn main() {
    clone_demo();
    let vec = vec![0,1,0,2,0,3,0,0];
    let big_int = BigInt::from_vec(vec.clone());
    println!("The vector is: {:?}", &vec);
    println!("As a BigInt, it has {} digits.", big_int.number_of_digits());
    println!("As a BigInt it has {} non-zero digits.", big_int.number_of_non_zero_digits());
    println!("As a BigInt the smallest digit is: {}", big_int.smallest_digit());
    println!("As a BigInt the biggest digit is: {}", big_int.biggest_digit());
}

// ## Mutation + aliasing considered harmful (part 2)
enum Variant {
    Number(i32),
    Text(String),
}
fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i32;
    match var {
        Variant::Number(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 1337;
}


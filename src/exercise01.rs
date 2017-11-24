fn vec_sum(vec: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for e in vec {
        sum = sum + e
    }

    sum
}

fn vec_print(vec: &Vec<i32>) {
    for e in vec {
        println!("{}", e)
    }
}

pub fn main() {
    let vec = vec![18,5,7,2,9,27];
    let sum = vec_sum(&vec);
    println!("Sum: {}", sum);
    vec_print(&vec);
}

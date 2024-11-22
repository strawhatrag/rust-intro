fn main() {
    let num = 10;
    let ans = is_even(num);
    println!("{}", ans);
}

// i is signed integer
fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}

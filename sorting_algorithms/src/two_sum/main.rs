fn main() {
    let n = 6;
    let x = 10;
    let mut a = [6, 5, 9, 0, 1, 3];
    print!("{}", TwoSum::search(&mut a, n, x));
}


struct TwoSum {}

impl TwoSum {
    fn search(a: &mut [i32], n: usize, x: i32) -> bool {
        a.sort();
        for i in 0..n {
            let exists = a.binary_search(&(x - a[i])).is_ok();
            if exists {return true;}
        }
        return false;
    }
}

fn main() {
    let mut a = [5, 2, 4, 6, 1, 3];
    InsertionSort::sort(&mut a);
    print!("[");
    for elem in a {
        print!(" {} ", elem);
    }
    print!("]");
}

struct InsertionSort {}

impl InsertionSort {
    fn sort(a: &mut [i32]) {
        for i in 1..a.len() {
            let key = a[i];
            let mut j = i;
            while j > 0 && a[j - 1] > key {
                a[j] = a[j - 1];
                j = j - 1;  // an usize can't be negative
            }
            a[j] = key;
        }
    }
}

fn main() {
    let mut a = [5, 2, 4, 6, 1, 3];
    SelectionSort::sort(&mut a);
    print!("[");
    for elem in a {
        print!(" {} ", elem);
    }
    print!("]");
}

struct SelectionSort {}

impl SelectionSort {
    fn sort(a: &mut [i32]) {
        let mut min: i32;
        let mut index: usize;
        let n: usize = a.len();
        for i in 0..n
        {
            min = a[i];
            index = i;
            for j in i..n
            {
                if a[j] < min
                {
                    min = a[j];
                    index = j;
                }
            }
            a[index] = a[i];
            a[i] = min;
        }
    }
}

fn main() {
    let n: usize = 6;
    let mut a = [6, 5, 9, 0, 1, 7];
    let num_inversions = NumInversions::count_inversions(&mut a, 0, n - 1);
    println!("{}", num_inversions);
}

struct NumInversions {}

impl NumInversions {
    fn count_inversions(a: &mut [i32], p: usize, r: usize) -> i32 {
        let mut num_inversions = 0;
        if p >= r { return num_inversions; }
        let q = (p + r) / 2;
        num_inversions += Self::count_inversions(a, p, q);
        num_inversions += Self::count_inversions(a, q + 1, r);
        num_inversions += Self::merge(a, p, q, r);
        return num_inversions
    }

    fn merge(a: &mut [i32], p: usize, q: usize, r: usize) -> i32 {
        let mut num_inversions: i32 = 0;
        let n_l = q - p + 1;
        let n_r = r - q;
        let mut l = vec![0; n_l];
        let mut r = vec![0; n_r];
        for i in 0..n_l {
            l[i] = a[p + i];
        }
        for j in 0..n_r {
            r[j] = a[q + j + 1];
        }
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut k = p;
        while i < n_l && j < n_r {
            if l[i] <= r[j] {
                a[k] = l[i];
                i += 1;
            } else {
                a[k] = r[j];
                num_inversions += (n_l - i) as i32;
                j += 1;
            }
            k += 1;
        }
        while i < n_l {
            a[k] = l[i];
            i += 1;
            k += 1;
        }
        while j < n_r {
            a[k] = r[j];
            j += 1;
            k += 1;
        }
        return num_inversions
    }
}

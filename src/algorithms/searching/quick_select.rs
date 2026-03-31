pub fn quick_select(list: &mut [i32], left: usize, right: usize, index: usize) -> i32 {
    if left == right {
        return list[left];
    }

    let p = partition(list, left, right);

    if index == p {
        list[p]
    } else if index < p {
        quick_select(list, left, p.saturating_sub(1), index)
    } else {
        quick_select(list, p + 1, right, index)
    }
}

fn partition(list: &mut [i32], left: usize, right: usize) -> usize {
    let pivot = list[right];
    let mut p = left;

    for i in left..right {
        if list[i] < pivot {
            list.swap(p, i);
            p += 1;
        }
    }
    list.swap(right, p);
    p
}

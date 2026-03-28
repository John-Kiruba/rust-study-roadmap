use std::cmp::Ordering;

fn binary_search<T: Ord>(search_item: &T, arr: &[T]) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let is_asc = is_asc_arr(arr);

    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        if match_val(search_item, arr, &mut left, &mut right, is_asc) {
            return Some(left);
        }
    }
    None
}

fn is_asc_arr<T: Ord>(arr: &[T]) -> bool {
    arr.len() > 0 && arr[0] < arr[arr.len() - 1]
}

fn match_val<T: Ord>(
    search_item: &T,
    arr: &[T],
    left: &mut usize,
    right: &mut usize,
    is_asc: bool,
) -> bool {
    let mid = *left + (*right - *left) / 2;

    let cmp_res = search_item.cmp(&arr[mid]);

    match (is_asc, cmp_res) {
        (true, Ordering::Less) | (false, Ordering::Greater) => {
            *right = mid;
        }
        (true, Ordering::Greater) | (false, Ordering::Less) => {
            *left = mid + 1;
        }
        (_, Ordering::Equal) => {
            *left = mid;
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_cases {
        ($($name:ident: $test_case:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (item, arr, expected) = $test_case;
                    assert_eq!(binary_search(&item, arr),expected);
                }
            )*
        };
    }

    test_cases! {
        empty: ("a", &[] as &[&str], None),
        one_item_found: ("a", &["a"], Some(0)),
        one_item_not_found: ("b", &["a"], None),search_strings_asc_start: ("a", &["a", "b", "c", "d", "google", "zoo"], Some(0)),
            search_strings_asc_middle: ("google", &["a", "b", "c", "d", "google", "zoo"], Some(4)),
            search_strings_asc_last: ("zoo", &["a", "b", "c", "d", "google", "zoo"], Some(5)),
            search_strings_asc_not_found: ("x", &["a", "b", "c", "d", "google", "zoo"], None),
            search_strings_desc_start: ("zoo", &["zoo", "google", "d", "c", "b", "a"], Some(0)),
            search_strings_desc_middle: ("google", &["zoo", "google", "d", "c", "b", "a"], Some(1)),
            search_strings_desc_last: ("a", &["zoo", "google", "d", "c", "b", "a"], Some(5)),
            search_strings_desc_not_found: ("x", &["zoo", "google", "d", "c", "b", "a"], None),
            search_ints_asc_start: (1, &[1, 2, 3, 4], Some(0)),
    }
}

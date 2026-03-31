//pub mod algorithms;
mod algorithms;
use algorithms::searching::quick_select::quick_select;
fn main() {
    let mut data = [2, 5, 1, 67, 1, 3, 2, 6];
    let kl = 3;
    let len = data.len();
    let target_index = len - kl;
    let result = quick_select(&mut data, 0, len - 1, target_index);
    println!(
        "The {}-th largest element in the data: {:?}, is: {}",
        kl, data, result
    );
}

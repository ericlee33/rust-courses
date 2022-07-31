// ========== demo1 ============
// fn main() {
//     let (a, mut b): (bool, bool) = (true, false);

//     println!("a: {}, b: {}", a, b);

//     b = true;

//     println!("a: {}, b: {}", a, b);

// }

// ========== demo2 ============
// use tutorials_1::same_level_utils;

// fn main() {
//     same_level_utils::print_abc_word();
// }

// ========== demo3 ============
use tutorials_1::utils::date;

fn main() {
    let current_time = date::get_current_system_time();

    println!("time is: {:?}", current_time);
}

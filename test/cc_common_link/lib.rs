#[no_mangle]
pub extern "C" fn four() -> i32 {
    let mut v = vec![];
    while v.iter().sum::<usize>() < 10 {
        v.push(v.len() + 1);
    }
    v.len() as i32
}

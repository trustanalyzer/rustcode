fn main() {
    let mut num = 5;

    let _r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("{:?}", *r2);
    }

    use std::slice;

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    //println!("{:?}", &values[..2]);
}

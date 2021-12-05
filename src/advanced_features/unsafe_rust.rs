pub fn main() {
  raw_pointers();
  unsafe_functions();
  safe_abstraction();
}

fn raw_pointers() {
  let mut num = 5;

  let r1 = &num as *const i32;
  let r2 = &mut num as *mut i32;

  unsafe {
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
  }
}

fn unsafe_functions() {
  unsafe fn dangerous() {}

  unsafe {
    dangerous();
  }
}

fn safe_abstraction() {
  fn basic_functionality() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
  }

  basic_functionality();

  use std::slice;

  fn split_at_mut(slice: &mut [i32; 4], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
      (
        slice::from_raw_parts_mut(ptr, mid),
        slice::from_raw_parts_mut(ptr.add(mid), len - mid),
      )
    }
  }

  fn safe_functionality() {
    let mut v = [1, 2, 3, 4];
    let r = &mut v;
    let (a, b) = split_at_mut(r, 2);

    assert_eq!(a, &mut [1, 2]);
    assert_eq!(b, &mut [3, 4]);
  }

  safe_functionality();
}

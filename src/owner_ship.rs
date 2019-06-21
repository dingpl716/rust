fn test(mut arr: [i32; 3]) {
    arr[2] = 5;
}

fn move_test(mut v: Vec<i32>) -> Vec<i32> {
    v.push(5);
    v
}

fn main() {
    let arr = [1, 2, 3];  // copy
    test(arr);
    println!("{:?}", arr);

    let v = vec![1,2,3]; // moved
    let v = move_test(v);
    println!("{:?}", v);
}


// =========================

fn test(mut arr: [i32; 3]) {
    arr[2] = 5;
}

fn move_test(mut v: Vec<i32>) {
    v.push(5);
}

fn main() {
    let arr = [1, 2, 3];  // copy
    test(arr);
    println!("{:?}", arr);

    let v = vec![1,2,3]; // moved
    move_test(v);
    println!("{:?}", v);
}

// ======================

fn test(arr: &mut [i32; 3]) {
    arr[2] = 5;
}

fn move_test(v: &mut Vec<i32>) {
    v.push(5);
}

fn main() {
    let mut arr = [1, 2, 3];  // copy
    test(&mut arr);
    println!("{:?}", arr);

    let mut v = vec![1,2,3]; // moved
    move_test(&mut v);
    println!("{:?}", v);
}


// =================

fn test(arr: &mut [i32; 3]) {
    arr[2] = 5;
}

fn move_test(v: &mut Vec<i32>) {
    v.push(5);
}

fn main() {
    let mut arr = [1, 2, 3];  // copy
    test(&mut arr);
    let _arr2 = &mut arr; // cannot borrow twice
    println!("{:?}", arr);

    let mut v = vec![1,2,3]; // moved
    move_test(&mut v);
    println!("{:?}", v);
}

// ======================================

fn test(p: (i32, String)) {
    print!("{:?}", p.1);
}

fn main() {
    let point = (1, String::from("hello"));
    test(point);
    println!("{:?}", point) // moved
}

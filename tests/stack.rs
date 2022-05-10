use rust_ds101::stack::Stack10;

#[test]
fn stack10_push() {
    let mut stack10 = Stack10::new();

    stack10.push(1);
    assert_eq!(stack10.len(), 1);
}

#[test]
fn stack10_pop() {
    let mut stack10 = Stack10::new();

    stack10.push(1);
    let e = stack10.pop();
    assert_eq!(e, 1);
}

#[test]
fn stack10_push_10() {
    let mut stack10 = Stack10::new();

    for i in 1..=10 {
        stack10.push(i as i64);
        assert_eq!(stack10.len(), i);
    }
}

#[test]
fn stack10_pop_10() {
    let mut stack10 = Stack10::new();

    for i in 1..=10 { stack10.push(i); }

    println!("{:?}", stack10);

    for i in (1..=10).rev() {
        assert_eq!(stack10.pop(), i)
    }
}

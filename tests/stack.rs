use rust_ds101::stack::Stack;

#[test]
fn stack_push() {
    let mut stack = Stack::new();

    stack.push(1);
    assert_eq!(stack.len(), 1);
}

#[test]
fn stack_pop() {
    let mut stack = Stack::new();

    stack.push(1);
    assert_eq!(stack.pop(), Some(1));
}

#[test]
fn stack_push_10() {
    let mut stack = Stack::new();

    for i in 1..=10 {
        stack.push(i);
        assert_eq!(stack.len(), i);
    }
}

#[test]
fn stack_pop_10() {
    let mut stack = Stack::new();

    for i in 1..=10 { stack.push(i); }

    for i in (1..=10).rev() {
        assert_eq!(stack.pop(), Some(i));
    }
}

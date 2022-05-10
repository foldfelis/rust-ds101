use rust_ds101::queue::Queue;

#[test]
fn queue_push() {
    let mut queue = Queue::new();

    queue.push(1);
    assert_eq!(queue.len(), 1);
}

#[test]
fn queue_pop() {
    let mut queue = Queue::new();

    queue.push(1);
    assert_eq!(queue.pop(), Some(1));
}

#[test]
fn queue_push_10() {
    let mut queue = Queue::new();

    for i in 1..=10 {
        queue.push(i);
        assert_eq!(queue.len(), i);
    }
}

#[test]
fn queue_pop_10() {
    let mut queue = Queue::new();

    for i in 1..=10 { queue.push(i); }

    for i in 1..=10 {
        assert_eq!(queue.pop(), Some(i));
    }
}

use behavior_tree::*;

struct Counter {
    value: i32,
}

impl StatefulAction<(), ()> for Counter {
    fn tick(&mut self, _data: &mut (), _props: &()) -> Status {
        if self.value == 0 {
            self.value += 1;
            Status::Running
        } else {
            Status::Success
        }
    }
}

fn inc_once(data: &mut Counter, _: &()) -> Status {
    if data.value % 2 == 0 {
        data.value += 1;
        Status::Running
    } else {
        data.value += 1;
        Status::Success
    }
}

#[test]
fn test_simple_action() {
    let mut bt: Behavior<Counter, ()> = Behavior::Action("inc_once", inc_once);

    let mut data = Counter { value: 0 };

    let (status, _) = bt.tick(0.0, &mut data, &());
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 1);

    let (status, _) = bt.tick(0.0, &mut data, &());
    assert_eq!(status, Status::Success);
    assert_eq!(data.value, 2);
}

#[test]
fn test_simple_sequence() {
    let mut bt: Behavior<Counter, ()> = sequence![Behavior::Action("inc_once", inc_once)];

    // S
    // |
    // A

    let mut data = Counter { value: 0 };

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 1);
    assert_eq!(debug_repr.cursor.index(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    assert_eq!(status, Status::Success);
    assert_eq!(data.value, 2);
    assert_eq!(debug_repr.cursor.index(), 1);
}

#[test]
fn test_stateful_action() {
    let mut bt: Behavior<(), ()> = sequence![
        Behavior::StatefulAction("inc_x".to_owned(), Box::new(Counter { value: 0 })),
        Behavior::StatefulAction("inc_y".to_owned(), Box::new(Counter { value: 0 }))
    ];

    //   Player visible?
    //  / \
    // S   Wait
    // |
    // X -> Y
    //
    // sequence(start_attack, wait, stop_attack)

    let mut data = ();

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    // dbg!(&debug_repr);
    // assert_eq!(data.x, 1);
    // assert_eq!(data.y, 0);
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    // dbg!(&debug_repr);
    // assert_eq!(data.x, 1);
    // assert_eq!(data.y, 1);
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 1);

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    // dbg!(&debug_repr);
    // assert_eq!(data.x, 1);
    // assert_eq!(data.y, 1);
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 2);

    // let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    // dbg!(&debug_repr);
    // assert_eq!(data.value, 4);
    // assert_eq!(status, Status::Success);
    // assert_eq!(debug_repr.cursor.index(), 1);
}

#[test]
fn test_nested_sequence() {
    struct DoubleCounter {
        x: i32,
        y: i32,
    }

    fn inc_x(data: &mut DoubleCounter, _: &()) -> Status {
        if data.x % 2 == 0 {
            data.x += 1;
            Status::Running
        } else {
            Status::Success
        }
    }

    fn inc_y(data: &mut DoubleCounter, _: &()) -> Status {
        if data.y % 2 == 0 {
            data.y += 1;
            Status::Running
        } else {
            Status::Success
        }
    }

    let mut bt: Behavior<DoubleCounter, ()> = sequence![
        Behavior::Action("inc_once_1", inc_x),
        Behavior::Action("inc_once_2", inc_y)
    ];

    // S
    // |
    // X -> Y
    //
    // sequence(start_attack, wait, stop_attack)

    let mut data = DoubleCounter { x: 0, y: 0 };

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    assert_eq!(status, Status::Running);
    assert_eq!(data.x, 1);
    assert_eq!(data.y, 0);
    assert_eq!(debug_repr.cursor.index(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    assert_eq!(status, Status::Running);
    assert_eq!(data.x, 1);
    assert_eq!(data.y, 1);
    assert_eq!(debug_repr.cursor.index(), 1);

    let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    assert_eq!(data.x, 1);
    assert_eq!(data.y, 1);
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 2);

    // let (status, debug_repr) = bt.tick(0.0, &mut data, &());
    // dbg!(&debug_repr);
    // assert_eq!(data.value, 4);
    // assert_eq!(status, Status::Success);
    // assert_eq!(debug_repr.cursor.index(), 1);
}

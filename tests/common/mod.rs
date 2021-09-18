use behavior_tree::*;

/// Node that will panic upon being dropped if it hasn't ticked
/// at least once.
///
/// Inspired by https://github.com/danieleades/aspen/blob/master/src/std_nodes/testing.rs
#[derive(Default)]
pub struct YesTick {
    pub ticked: bool,
}

impl<T> StatefulAction<T> for YesTick {
    fn tick(&mut self, _data: &mut T) -> Status {
        self.ticked = true;
        Status::Success
    }

    fn reset(&mut self) {
        // no-op
    }
}

impl Drop for YesTick {
    fn drop(&mut self) {
        if !self.ticked {
            panic!("YesTick dropped without being ticked");
        }
    }
}

/// Node that will panic when it ticks.
///
/// Inspired by https://github.com/danieleades/aspen/blob/master/src/std_nodes/testing.rs
#[derive(Default)]
pub struct NoTick;

impl<T> StatefulAction<T> for NoTick {
    fn tick(&mut self, _data: &mut T) -> Status {
        panic!("NoTick node should never be ticked");
    }

    fn reset(&mut self) {
        panic!("Since NoTick should never be ticked, it should also never be reset");
    }
}

pub struct Counter {
    pub value: i32,
}

impl StatefulAction<()> for Counter {
    fn tick(&mut self, _data: &mut ()) -> Status {
        if self.value == 0 {
            self.value += 1;
            Status::Running
        } else {
            Status::Success
        }
    }

    fn reset(&mut self) {
        self.value = 0;
    }
}

pub fn inc_pingpong(data: &mut Counter) -> Status {
    if data.value % 2 == 0 {
        data.value += 1;
        Status::Running
    } else {
        data.value += 1;
        Status::Success
    }
}

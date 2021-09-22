use behavior_tree::*;

pub struct EvenCounter {
    pub value: i32,
}

impl StatefulAction<()> for EvenCounter {
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

pub fn inc_pingpong(data: &mut EvenCounter) -> Status {
    if data.value % 2 == 0 {
        data.value += 1;
        Status::Running
    } else {
        data.value += 1;
        Status::Success
    }
}

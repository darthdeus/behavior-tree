use crate::prelude::*;

pub struct Node<T> {
    pub name: Option<String>,
    pub behavior: Behavior<T>,
    pub status: Status,
}

impl<T> Node<T> {
    fn new(behavior: Behavior<T>) -> Node<T> {
        Node {
            name: None,
            behavior,
            status: Status::Running,
        }
    }

    pub fn new_named(name: String, behavior: Behavior<T>) -> Node<T> {
        Node {
            name: Some(name),
            behavior,
            status: Status::Running,
        }
    }

    pub fn action(name: &str, func: fn(&mut T) -> Status) -> Node<T> {
        Self::new_named(name.to_owned(), Behavior::Action(name.to_owned(), func))
    }

    pub fn action_success(name: &str, func: fn(&mut T) -> ()) -> Node<T> {
        Self::new_named(
            name.to_owned(),
            Behavior::ActionSuccess(name.to_owned(), func),
        )
    }

    pub fn stateful_action(name: &str, func: Box<dyn StatefulAction<T>>) -> Node<T> {
        Self::new_named(
            name.to_owned(),
            Behavior::StatefulAction(name.to_owned(), func),
        )
    }

    pub fn sequence(nodes: Vec<Node<T>>) -> Node<T> {
        Self::new(Behavior::Sequence(0, nodes))
    }

    pub fn select(nodes: Vec<Node<T>>) -> Node<T> {
        Self::new(Behavior::Select(0, nodes))
    }

    pub fn cond(name: &str, cond: fn(&T) -> bool, success: Node<T>, failure: Node<T>) -> Node<T> {
        Self::new_named(
            name.to_owned(),
            Behavior::Cond(name.to_owned(), cond, Box::new(success), Box::new(failure)),
        )
    }

    pub fn wait(time: f64) -> Node<T> {
        Self::new(Behavior::Wait {
            curr: time,
            max: time,
        })
    }

    pub fn tick(&mut self, delta: f64, context: &mut T) -> (Status, DebugRepr) {
        if self.status == Status::Success || self.status == Status::Failure {
            self.behavior.reset();
        }

        let (status, repr) = self.behavior.tick(delta, context);
        self.status = status;
        (status, repr)
    }
}

use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub struct Node<T> {
    pub name: Option<String>,
    pub behavior: Behavior<T>,
    pub status: Status,
    pub collapse_as: Option<String>,
}

impl<T> Node<T> {
    fn new(behavior: Behavior<T>) -> Node<T> {
        Node {
            name: None,
            behavior,
            status: Status::Running,
            collapse_as: None,
        }
    }

    pub fn new_named(name: String, behavior: Behavior<T>) -> Node<T> {
        Node {
            name: Some(name),
            behavior,
            status: Status::Running,
            collapse_as: None,
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

    pub fn named_sequence(name: &str, nodes: Vec<Node<T>>) -> Node<T> {
        Self::new_named(name.to_owned(), Behavior::Sequence(0, nodes))
    }

    pub fn select(nodes: Vec<Node<T>>) -> Node<T> {
        Self::new(Behavior::Select(0, nodes))
    }

    pub fn named_select(name: &str, nodes: Vec<Node<T>>) -> Node<T> {
        Self::new_named(name.to_owned(), Behavior::Select(0, nodes))
    }

    pub fn cond(name: &str, cond: fn(&T) -> bool, success: Node<T>, failure: Node<T>) -> Node<T> {
        Self::new_named(
            name.to_owned(),
            Behavior::Cond(
                name.to_owned(),
                cond,
                Rc::new(RefCell::new(success)),
                Rc::new(RefCell::new(failure)),
            ),
        )
    }

    pub fn wait(time: f64) -> Node<T> {
        Self::new(Behavior::Wait {
            curr: time,
            max: time,
        })
    }

    pub fn named_while_single(name: &str, cond: fn(&T) -> bool, child: Node<T>) -> Node<T> {
        Self::new_named(
            name.to_owned(),
            Behavior::While(cond, Rc::new(RefCell::new(child))),
        )
    }

    pub fn while_single(cond: fn(&T) -> bool, child: Node<T>) -> Node<T> {
        Self::new(Behavior::While(cond, Rc::new(RefCell::new(child))))
    }

    pub fn collapse(self, _desc: &str) -> Node<T> {
        self
        // TODO: re-enable once behavior-tree-egui catches up
        // Self {
        //     collapse_as: Some(desc.to_owned()),
        //     ..self
        // }
    }

    pub fn tick(&mut self, delta: f64, context: &mut T) -> (Status, DebugRepr) {
        if self.status == Status::Success || self.status == Status::Failure {
            self.behavior.reset();
        }

        let (status, repr) = self.behavior.tick(delta, context);
        self.status = status;
        (status, repr)
    }

    pub fn children(&self) -> Vec<&Node<T>> {
        match self.behavior {
            Behavior::Wait { .. } => vec![],
            _ => vec![],
            // Behavior::Cond(_, _, positive, negative) => vec![positive, negative],
            Behavior::Sequence(_, ref seq) => seq.iter().collect(),
            Behavior::Select(_, _) => todo!(),
            Behavior::Action(_, _) => todo!(),
            Behavior::ActionSuccess(_, _) => todo!(),
            Behavior::StatefulAction(_, _) => todo!(),
            Behavior::While(_, _) => todo!(),
        }
    }

    pub fn to_debug(&self) -> TreeRepr {
        let mut repr = match &self.collapse_as {
            Some(collapse_text) => TreeRepr::new(collapse_text, vec![]),
            None => {
                match &self.behavior {
                    Behavior::Wait { curr, max } => TreeRepr::new("Wait", vec![])
                        .with_detail(format!("curr={}, max={}", curr, max)),
                    Behavior::Cond(name, _cond, a, b) => {
                        TreeRepr::new("Cond", vec![a.borrow().to_debug(), b.borrow().to_debug()])
                            .with_detail(name.clone())
                    }
                    Behavior::Sequence(_, seq) => TreeRepr::new(
                        if let Some(ref name) = self.name {
                            format!("Sequence {}", name)
                        } else {
                            "Sequence".to_string()
                        },
                        seq.iter().map(|x| x.to_debug()).collect(),
                    ),
                    Behavior::Select(_, seq) => TreeRepr::new(
                        if let Some(ref name) = self.name {
                            format!("Select {}", name)
                        } else {
                            "Select".to_string()
                        },
                        seq.iter().map(|x| x.to_debug()).collect(),
                    ),
                    Behavior::Action(name, _) => {
                        TreeRepr::new("Action", vec![]).with_detail(name.clone())
                    }
                    Behavior::ActionSuccess(name, _) => {
                        TreeRepr::new("ActionSuccess", vec![]).with_detail(name.clone())
                    }
                    Behavior::StatefulAction(name, _) => {
                        TreeRepr::new("StatefulAction", vec![]).with_detail(name.clone())
                    }
                    // Behavior::While(_, x) => TreeRepr::new("While", vec![x.to_debug()]),
                    // TODO: add to detail
                    Behavior::While(_, x) => x.borrow().to_debug(),
                }
            }
        };

        repr.status = self.status;
        repr
    }
}

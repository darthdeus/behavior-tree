use crate::maybe_profile_function;
use crate::prelude::*;
use std::{cell::RefCell, rc::Rc};

pub trait StatefulAction<T> {
    fn tick(&mut self, data: &mut T) -> Status;
    fn reset(&mut self);
}

pub struct BehaviorTree<T> {
    pub tree: Rc<RefCell<Node<T>>>,
}

impl<T> BehaviorTree<T> {
    pub fn new(root: Node<T>) -> Self {
        let root = Rc::new(RefCell::new(root));

        Self { tree: root }
    }
}

pub enum Behavior<T> {
    Wait {
        curr: f64,
        max: f64,
    },

    RandomWait {
        curr: f64,
        curr_max: f64,
        max: f64,
    },

    Cond(
        String,
        // Rc<dyn Fn(&mut T, &P) -> bool>,
        fn(&T) -> bool,
        Rc<RefCell<Node<T>>>,
        Rc<RefCell<Node<T>>>,
    ),

    Sequence(usize, Vec<Rc<RefCell<Node<T>>>>),
    Select(usize, Vec<Rc<RefCell<Node<T>>>>),

    Action(String, fn(&mut T) -> Status),
    ActionSuccess(String, fn(&mut T) -> ()),

    StatefulAction(String, Box<dyn StatefulAction<T>>),
    // StatefulAction(String, fn(&mut T, &P) -> Status),

    // Invert(Rc<Behavior<T>>),
    // AlwaysSucceed(Rc<Behavior<T>>),
    // Condition(Rc<dyn Fn(f64, &mut T, &P) -> bool>, Rc<Behavior<T>>),
    // WaitForever,
    // Action(T),
    While(Box<dyn Fn(&T) -> bool>, Rc<RefCell<Node<T>>>),
}

fn sequence<T>(
    delta: f64,
    context: &mut T,
    is_sequence: bool,
    current: &mut usize,
    xs: &mut Vec<Rc<RefCell<Node<T>>>>,
) -> Status {
    maybe_profile_function!();

    let (status_positive, status_negative) = if is_sequence {
        (Status::Success, Status::Failure)
    } else {
        (Status::Failure, Status::Success)
    };

    let mut repr_string = String::new();
    let mut status = status_positive;

    let len = xs.len();

    for i in 0..*current {
        if xs[i].borrow_mut().recheck_condition(context, is_sequence) {
            *current = i;
            for j in (i + 1)..len {
                if j < len {
                    xs[j].borrow_mut().reset();
                }
            }
            // TODO: add a test that verifies that the break is needed
            break;
        }
    }

    // Resetting state
    if *current == len {
        *current = 0;
    }

    while *current < len {
        let mut x = xs[*current].borrow_mut();

        if x.status == Status::Success || x.status == Status::Failure {
            // *current += 1;
            // continue;
            x.reset();
        }
        let res = x.tick(delta, context);

        if res == status_positive {
            *current += 1;
            repr_string += "+";
        } else if res == status_negative {
            status = status_negative;
            repr_string += "-";
            break;
        } else {
            status = Status::Running;
            repr_string += ".";
            break;
        }

        // match x.tick(delta, context) {
        //     (Status::Success, repr) => {
        //     }
        //     (Status::Failure, repr) => {
        //         // return (Status::Failure, DebugRepr::new("Sequence", Status::Failure))
        //     }
        //     (Status::Running, repr) => {
        //         // return (Status::Running, DebugRepr::new("Sequence", Status::Running))
        //     }
        // }
    }

    status
}
//
// for (i, x) in xs.iter_mut().enumerate() {
//     match x.tick(delta, context) {
//         (Status::Success, repr) => {
//             if i < len - 1 {
//                 index += 1;
//             }
//             repr_string += "+";
//             child_repr = Some(repr);
//         }
//         (Status::Failure, repr) => {
//             status = Status::Failure;
//             repr_string += "-";
//             child_repr = Some(repr);
//             break;
//             // return (Status::Failure, DebugRepr::new("Sequence", Status::Failure))
//         }
//         (Status::Running, repr) => {
//             status = Status::Running;
//             repr_string += ".";
//             child_repr = Some(repr);
//             break;
//             // return (Status::Running, DebugRepr::new("Sequence", Status::Running))
//         }
//     }
// }

impl<T> Behavior<T> {
    pub fn tick(&mut self, delta: f64, context: &mut T) -> Status {
        maybe_profile_function!();

        let _status = match self {
            Behavior::Wait {
                ref mut curr,
                max: _,
            } => {
                *curr -= delta;
                let status = if *curr <= 0.0 {
                    Status::Success
                } else {
                    Status::Running
                };

                return status;
            }

            Behavior::RandomWait {
                ref mut curr,
                curr_max: _,
                max: _,
            } => {
                *curr -= delta;
                let status = if *curr <= 0.0 {
                    Status::Success
                } else {
                    Status::Running
                };

                return status;
            }

            Behavior::Cond(_, cond, a, b) => {
                let c = cond(context);

                let status = if c {
                    a.borrow_mut().tick(delta, context)
                } else {
                    b.borrow_mut().tick(delta, context)
                };

                return status;
            }

            Behavior::Sequence(ref mut current, xs) => {
                return sequence(delta, context, true, current, xs)
            }

            Behavior::Select(ref mut current, xs) => {
                return sequence(delta, context, false, current, xs)
            }

            Behavior::Action(_, action) => {
                let status = action(context);
                return status;
            }

            Behavior::ActionSuccess(_, action) => {
                let _ = action(context);
                return Status::Success;
            }

            // TODO: state reset?
            Behavior::StatefulAction(_, action) => {
                return action.tick(context);
            }

            Behavior::While(cond, behavior) => {
                if cond(context) {
                    return behavior.borrow_mut().tick(delta, context);
                } else {
                    return Status::Failure;
                }
            } //     Status::Success => Status::Failure,
              //     Status::Failure => Status::Success,
              //     Status::Running => Status::Running,
              // },
              // Behavior::AlwaysSucceed(b) => match b.tick(delta, context).0 {
              //     Status::Success | Status::Failure => Status::Success,
              //     Status::Running => Status::Running,
              // },

              // Behavior::Condition(cond, action) => {
              //     if cond(delta, context) {
              //         action.tick(delta, context).0
              //     } else {
              //         Status::Failure
              //     }
              // }

              // Behavior::StatefulAction(_) => todo!(),
              // _ => todo!(),
              //             Behavior::Select(xs) => {
              //                 for x in xs.iter_mut() {
              //                     match x.tick(delta, context).0 {
              //                         Status::Success => {
              //                             return (Status::Success, DebugRepr::new("Select", Status::Success))
              //                         }
              //                         Status::Failure => {
              //                             return (Status::Running, DebugRepr::new("Select", Status::Running))
              //                         }
              //                         Status::Running => {}
              //                     }
              //                 }
              //
              //                 Status::Failure
              //             }
        };

        // (status, DebugRepr::new("X", status))
    }

    pub fn reset(&mut self) {
        maybe_profile_function!();

        match self {
            Behavior::Wait { ref mut curr, max } => {
                *curr = *max;
            }
            Behavior::RandomWait {
                ref mut curr,
                ref mut curr_max,
                max,
            } => {
                *curr_max = rand::random::<f64>() * *max;
                *curr = *curr_max;
            }
            Behavior::Sequence(ref mut idx, nodes) => {
                *idx = 0;
                for node in nodes.iter_mut() {
                    node.borrow_mut().reset();
                }
            }
            Behavior::Select(ref mut idx, nodes) => {
                *idx = 0;
                for node in nodes.iter_mut() {
                    node.borrow_mut().reset();
                }
            }
            Behavior::StatefulAction(_name, ref mut state) => {
                state.reset();
            }

            Behavior::While(_, node) => node.borrow_mut().reset(),
            _ => {}
        }
    }
}

impl<T> core::fmt::Debug for Behavior<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        maybe_profile_function!();

        match self {
            Behavior::Wait{curr, max} => {
                f.debug_struct("Wait").field("current", curr).field("max", max);
            }
            Behavior::Action(name, _fn) => {
                f.debug_struct("Action").field("name", name);
            }
            // Behavior::StatefulAction() => todo!(),

            _ => {}
            // Behavior::Invert(_) => todo!(),
            // Behavior::AlwaysSucceed(_) => todo!(),
            // Behavior::Cond(_, _, _) => todo!(),
            // Behavior::Sequence(_) => todo!(),
            // Behavior::Select(_) => todo!(),
            // Behavior::Condition(_, _) => todo!(),
            // Behavior::ActionSuccess(_) => todo!(),
            // Behavior::StatefulAction(_) => todo!(),
        };

        Ok(())
    }
}

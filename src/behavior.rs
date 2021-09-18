use crate::prelude::*;

pub trait StatefulAction<T> {
    fn tick(&mut self, data: &mut T) -> Status;
    fn reset(&mut self);
}

pub struct BehaviorTree<T> {
    pub tree: Node<T>,
    pub debug: TreeRepr,
}

pub enum Behavior<T> {
    Wait {
        curr: f64,
        max: f64,
    },
    Cond(
        String,
        // Box<dyn Fn(&mut T, &P) -> bool>,
        fn(&T) -> bool,
        Box<Node<T>>,
        Box<Node<T>>,
    ),

    Sequence(usize, Vec<Node<T>>),
    Select(usize, Vec<Node<T>>),

    Action(String, fn(&mut T) -> Status),
    ActionSuccess(String, fn(&mut T) -> ()),

    StatefulAction(String, Box<dyn StatefulAction<T>>),
    // StatefulAction(String, fn(&mut T, &P) -> Status),

    // Invert(Box<Behavior<T>>),
    // AlwaysSucceed(Box<Behavior<T>>),
    // Condition(Box<dyn Fn(f64, &mut T, &P) -> bool>, Box<Behavior<T>>),
    // WaitForever,
    // Action(T),
    // While(Box<Behavior<T>>, Box<Behavior<T>>),
}

fn sequence<T>(
    delta: f64,
    context: &mut T,
    is_sequence: bool,
    current: &mut usize,
    xs: &mut Vec<Node<T>>,
) -> (Status, DebugRepr) {
    let (status_positive, status_negative) = if is_sequence {
        (Status::Success, Status::Failure)
    } else {
        (Status::Failure, Status::Success)
    };

    let mut repr_string = String::new();
    let mut status = status_positive;
    let mut child_repr = None;

    let len = xs.len();

    // Resetting state
    if *current == len {
        *current = 0;
    }

    while *current < len {
        let x = &mut xs[*current];

        if x.status == Status::Success || x.status == Status::Failure {
            x.behavior.reset();
        }
        let (res, repr) = x.tick(delta, context);

        if res == status_positive {
            *current += 1;
            repr_string += "+";
            child_repr = Some(repr);
        } else if res == status_negative {
            status = status_negative;
            repr_string += "-";
            child_repr = Some(repr);
            break;
        } else {
            status = Status::Running;
            repr_string += ".";
            child_repr = Some(repr);
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

    let mut repr = DebugRepr::new(
        "Sequence",
        Cursor::Index(
            *current,
            Box::new(child_repr.expect("Sequence must have a child repr since it's non-empty")),
        ),
        status,
    );
    repr.params = Some(repr_string);

    (status, repr)
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
    pub fn tick(&mut self, delta: f64, context: &mut T) -> (Status, DebugRepr) {
        let _status = match self {
            Behavior::Wait {
                ref mut curr,
                max: _,
            } => {
                *curr -= delta;
                let status = if *curr <= 0.0 {
                    trace!("timer reset");
                    // *curr = *max;
                    Status::Success
                } else {
                    Status::Running
                };

                return (status, DebugRepr::new("Wait", Cursor::Leaf, status));
            }

            Behavior::Cond(s, cond, a, b) => {
                let c = cond(context);

                let (status, child_repr) = if c {
                    a.tick(delta, context)
                } else {
                    b.tick(delta, context)
                };

                // let mut repr = DebugRepr::new("If", Cursor::Condition(c), status);
                let mut repr = DebugRepr::new(
                    "If",
                    Cursor::Index(if c { 0 } else { 1 }, Box::new(child_repr)),
                    status,
                )
                .with_override(c);

                repr.params = Some(format!("true? = {:?} ... str = {:?}", c, s));
                return (status, repr);
            }

            Behavior::Sequence(ref mut current, xs) => {
                return sequence(delta, context, true, current, xs)
            }

            Behavior::Select(ref mut current, xs) => {
                return sequence(delta, context, false, current, xs)
            }

            Behavior::Action(name, action) => {
                let status = action(context);
                return (status, DebugRepr::new(name, Cursor::Leaf, status));
            }

            Behavior::ActionSuccess(name, action) => {
                let _ = action(context);
                return (
                    Status::Success,
                    DebugRepr::new(name, Cursor::Leaf, Status::Success),
                );
            }

            // TODO: state reset?
            Behavior::StatefulAction(name, action) => {
                let status = action.tick(context);
                return (status, DebugRepr::new(name, Cursor::Leaf, status));
            } // Behavior::Invert(b) => match b.tick(delta, context).0 {
              //     Status::Success => Status::Failure,
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
        match self {
            Behavior::Wait { ref mut curr, max } => {
                *curr = *max;
            }
            Behavior::Sequence(ref mut idx, _) => {
                *idx = 0;
            }
            Behavior::StatefulAction(_name, ref mut state) => {
                state.reset();
            }
            _ => {}
        }
    }

    pub fn to_debug(&self) -> TreeRepr {
        match self {
            Behavior::Wait { curr, max } => {
                TreeRepr::new("Wait", vec![]).with_detail(format!("curr={}, max={}", curr, max))
            }
            Behavior::Cond(name, _cond, a, b) => {
                TreeRepr::new("Cond", vec![a.behavior.to_debug(), b.behavior.to_debug()])
                    .with_detail(name.clone())
            }
            Behavior::Sequence(_, seq) => TreeRepr::new(
                "Sequence",
                seq.iter().map(|x| x.behavior.to_debug()).collect(),
            ),
            Behavior::Select(_, seq) => TreeRepr::new(
                "Sequence",
                seq.iter().map(|x| x.behavior.to_debug()).collect(),
            ),
            Behavior::Action(name, _) => TreeRepr::new("Action", vec![]).with_detail(name.clone()),
            Behavior::ActionSuccess(name, _) => {
                TreeRepr::new("ActionSuccess", vec![]).with_detail(name.clone())
            }
            Behavior::StatefulAction(name, _) => {
                TreeRepr::new("StatefulAction", vec![]).with_detail(name.clone())
            }
        }
    }
}

impl<T> core::fmt::Debug for Behavior<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

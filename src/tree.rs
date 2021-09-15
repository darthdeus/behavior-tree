use crate::types::*;

pub struct BehaviorTree<T, P> {
    pub tree: Behavior<T, P>,
    pub debug: TreeRepr,
}

pub enum Behavior<T, P> {
    Wait(f64, f64),
    If(
        String,
        // Box<dyn Fn(&mut T, &P) -> bool>,
        fn(&mut T, &P) -> bool,
        Box<Behavior<T, P>>,
        Box<Behavior<T, P>>,
    ),

    // TODO: store cursor here + continue from where left off
    Sequence(Vec<Behavior<T, P>>),
    // Select(Vec<Behavior<T, P>>),
    Action(String, fn(&mut T, &P) -> Status),
    ActionSuccess(String, fn(&mut T, &P) -> ()),
    // Invert(Box<Behavior<T, P>>),
    // AlwaysSucceed(Box<Behavior<T, P>>),
    // StatefulAction(Box<dyn Fn(&mut T, &P) -> Status>),
    // Condition(Box<dyn Fn(f64, &mut T, &P) -> bool>, Box<Behavior<T, P>>),
    // WaitForever,
    // Action(T),
    // While(Box<Behavior<T>>, Box<Behavior<T>>),
}

impl<T, P> Behavior<T, P> {
    pub fn tick(&mut self, delta: f64, context: &mut T, props: &P) -> (Status, DebugRepr) {
        let _status = match self {
            Behavior::Wait(t_max, ref mut t) => {
                *t -= delta;
                let status = if *t < 0.0 {
                    *t = *t_max;
                    Status::Success
                } else {
                    Status::Running
                };

                return (status, DebugRepr::new("Wait", Cursor::Leaf, status));
            }

            Behavior::If(s, cond, a, b) => {
                let c = cond(context, props);

                let (status, child_repr) = if c {
                    a.tick(delta, context, props)
                } else {
                    b.tick(delta, context, props)
                };

                // let mut repr = DebugRepr::new("If", Cursor::Condition(c), status);
                let mut repr = DebugRepr::new(
                    "If",
                    Cursor::Index(if c { 0 } else { 1 }, Box::new(child_repr)),
                    status,
                );
                repr.params = Some(format!("true? = {:?} ... str = {:?}", c, s));
                return (status, repr);
            }

            Behavior::Sequence(xs) => {
                let mut repr_string = String::new();
                let mut status = Status::Success;
                let mut index = 0;
                let mut child_repr = None;

                for x in xs.iter_mut() {
                    match x.tick(delta, context, props) {
                        (Status::Success, repr) => {
                            repr_string += "+";
                            child_repr = Some(repr);
                        }
                        (Status::Failure, repr) => {
                            status = Status::Failure;
                            repr_string += "-";
                            child_repr = Some(repr);
                            break;
                            // return (Status::Failure, DebugRepr::new("Sequence", Status::Failure))
                        }
                        (Status::Running, repr) => {
                            status = Status::Running;
                            repr_string += ".";
                            child_repr = Some(repr);
                            break;
                            // return (Status::Running, DebugRepr::new("Sequence", Status::Running))
                        }
                    }

                    index += 1;
                }

                let mut repr = DebugRepr::new(
                    "Sequence",
                    Cursor::Index(index, Box::new(child_repr.unwrap())),
                    status,
                );
                repr.params = Some(repr_string);
                return (status, repr);
            }

            Behavior::Action(name, action) => {
                let status = action(context, props);
                return (status, DebugRepr::new(name, Cursor::Leaf, status));
            }
            Behavior::ActionSuccess(name, action) => {
                let _ = action(context, props);
                return (
                    Status::Success,
                    DebugRepr::new(name, Cursor::Leaf, Status::Success),
                );
            } // Behavior::Invert(b) => match b.tick(delta, context, props).0 {
              //     Status::Success => Status::Failure,
              //     Status::Failure => Status::Success,
              //     Status::Running => Status::Running,
              // },
              // Behavior::AlwaysSucceed(b) => match b.tick(delta, context, props).0 {
              //     Status::Success | Status::Failure => Status::Success,
              //     Status::Running => Status::Running,
              // },

              // Behavior::Condition(cond, action) => {
              //     if cond(delta, context, props) {
              //         action.tick(delta, context, props).0
              //     } else {
              //         Status::Failure
              //     }
              // }

              // Behavior::StatefulAction(_) => todo!(),
              // _ => todo!(),
              //             Behavior::Select(xs) => {
              //                 for x in xs.iter_mut() {
              //                     match x.tick(delta, context, props).0 {
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

    pub fn to_debug(&self) -> TreeRepr {
        match self {
            Behavior::Wait(curr, max) => {
                TreeRepr::new("Wait", vec![]).with_detail(format!("curr={}, max={}", curr, max))
            }
            Behavior::If(name, _cond, a, b) => {
                TreeRepr::new("If", vec![a.to_debug(), b.to_debug()])
                    .with_detail(format!("{}", name))
            }
            Behavior::Sequence(seq) => {
                TreeRepr::new("Sequence", seq.iter().map(|x| x.to_debug()).collect())
            }
            Behavior::Action(name, _) => {
                TreeRepr::new("Action", vec![]).with_detail(format!("{}", name))
            }
            Behavior::ActionSuccess(name, _) => {
                TreeRepr::new("ActionSuccess", vec![]).with_detail(format!("{}", name))
            }
        }
    }
}


impl<T, P> core::fmt::Debug for Behavior<T, P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Behavior::Wait(curr, max) => {
                f.debug_struct("Wait").field("current", curr).field("max", max);
            }

            _ => {}
            // Behavior::Invert(_) => todo!(),
            // Behavior::AlwaysSucceed(_) => todo!(),
            // Behavior::If(_, _, _) => todo!(),
            // Behavior::Sequence(_) => todo!(),
            // Behavior::Select(_) => todo!(),
            // Behavior::Condition(_, _) => todo!(),
            // Behavior::Action(_) => todo!(),
            // Behavior::ActionSuccess(_) => todo!(),
            // Behavior::StatefulAction(_) => todo!(),
        };

        Ok(())
    }
}

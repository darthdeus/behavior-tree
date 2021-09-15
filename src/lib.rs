pub use crate::macros::*;
pub use crate::tree::*;
pub use crate::types::*;

mod macros;
mod tree;
mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derp() {
        struct Data {
            counter: i32,
        }

        let mut bt: Behavior<Data, ()> = sequence![action!("inc_once", |data, _| {
            if data.counter == 0 {
                data.counter += 1;
                Status::Running
            } else {
                Status::Success
            }
        })];

        let mut data = Data { counter: 0 };

        let (status, debug_repr) = bt.tick(0.0, &mut data, &());
        // dbg!(&debug_repr);
        assert_eq!(status, Status::Running);
        assert_eq!(debug_repr.cursor.index(), 0);

        let (status, debug_repr) = bt.tick(0.0, &mut data, &());
        // dbg!(&debug_repr);
        assert_eq!(status, Status::Success);
        assert_eq!(debug_repr.cursor.index(), 0);
    }
}

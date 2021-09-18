use behavior_tree::*;
mod common;
use crate::common::*;

#[test]
#[should_panic]
fn test_yes_tick_panics_without_tick() {
    let _bt: Node<()> = Node::stateful_action("yes", Box::new(YesTick::default()));
}

#[test]
fn test_yes_tick_likes_being_ticked() {
    let mut bt: Node<()> = Node::stateful_action("yes", Box::new(YesTick::default()));

    bt.tick(0.0, &mut ());
}


#[test]
fn test_no_tick_without_tick() {
    let _bt: Node<()> = Node::stateful_action("no", Box::new(NoTick::default()));
}

#[test]
#[should_panic]
fn test_no_tick_crash_with_tick() {
    let mut bt: Node<()> = Node::stateful_action("no", Box::new(NoTick::default()));

    bt.tick(0.0, &mut ());
}


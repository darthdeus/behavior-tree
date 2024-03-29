# `behavior-tree` for Rust!

[![Crates.io](https://img.shields.io/crates/v/behavior-tree)](https://crates.io/crates/behavior-tree)
[![Crates.io (latest)](https://img.shields.io/crates/d/behavior-tree)](https://crates.io/crates/behavior-tree)
[![Crates.io](https://img.shields.io/crates/l/behavior-tree)](https://crates.io/crates/behavior-tree)
[![docs.rs](https://img.shields.io/docsrs/behavior-tree)](https://docs.rs/behavior-tree)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/darthdeus/behavior-tree/rust.yml)](https://github.com/darthdeus/behavior-tree/actions)
[![GitHub commit activity](https://img.shields.io/github/commit-activity/m/darthdeus/behavior-tree)](https://github.com/darthdeus/behavior-tree/commits/master)
[![GitHub branch checks state](https://img.shields.io/github/checks-status/darthdeus/behavior-tree/master)](https://github.com/darthdeus/behavior-tree/actions)

**USE AT YOUR OWN RISK. This crate is under heavy development at the moment and a lot of the APIs will change often and without any notice. Performance is also terrible right now.**

Implemented nodes:

- Sequence - execute child nodes in a sequence until one of them fails.
- Select - execute child nodes in a sequence until one of them succeeds.
- While - execute a child node only when a condition is true.
- Wait - constant time delay.
- RandomWait - random time delay with a defined max.
- Action - generic user-defined action.
- StatefulAction - generic user-defined action which manages its own state in addition to the tree-wide Blackboard.
- Cond - checks a condition and executes either the `positive` or `negative` child.

Almost all of the behaviors have test coverage including a few of the edge cases, but it is by no means exhaustive yet.

**There are a few quirks that need to be figured out, especially with respect to debugging/visualization, which will be stabilized before version `0.1`.**

## Other behavior tree crates

There are a few other crates that implement behavior trees (listed below).
This library is inspired by all of them, as well as the [Behavior Tree Starter
Kit from Game AI
Pro](https://www.gameaipro.com/GameAIPro/GameAIPro_Chapter06_The_Behavior_Tree_Starter_Kit.pdf).

- https://crates.io/crates/piston-ai_behavior
- https://github.com/pistondevelopers/ai_behavior

- https://crates.io/crates/aspen
- https://gitlab.com/neachdainn/aspen

- https://crates.io/crates/stackbt_behavior_tree
- https://github.com/eaglgenes101/stackbt

## Who uses this?

The code was originally extracted from [BITGUN](https://store.steampowered.com/app/1673940/BITGUN/), which continues to use it as it's being developed open source. If you're using this crate in your game [do open a PR](https://github.com/darthdeus/behavior-tree/pulls) so we can list your game here as well!

//! Common components for building Kubernetes operators
//!
//! This crate contains the core building blocks to allow users to build
//! controllers/operators/watchers that need to synchronize/reconcile kubernetes
//! state.
//!
//! Newcomers are recommended to start with the [`Controller`] builder, which gives an
//! opinionated starting point that should be appropriate for simple operators, but all
//! components are designed to be usable á la carte if your operator doesn't quite fit that mold.

#![deny(unsafe_code)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
// Makes for confusing SNAFU context selectors
#![allow(clippy::pub_enum_variant_names)]
// Triggered by many derive macros (kube-derive, derivative)
#![allow(clippy::default_trait_access)]
#![allow(clippy::type_repetition_in_bounds)]

pub mod controller;
pub mod reflector;
pub mod scheduler;
pub mod utils;
pub mod watcher;

pub use controller::{applier, Controller};
pub use reflector::reflector;
pub use scheduler::scheduler;
pub use watcher::watcher;

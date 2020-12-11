//! Thesis provides the `Experiment` struct, which represents an experiment to
//! run which compares the return values of multiple methods for accomplishing
//! the same task.
//!
//! Let's imagine that we already have a function called `load_data_from_db`,
//! which loads some data from a database. We want to refactor this to instead
//! load the same data from redis. We write a new function called
//! `load_data_from_redis` to accomplish the same task, but with redis instead of
//! a DB. We want to use the redis version on only a very small percentage of
//! traffic, say 0.5% of incoming requests, and we want to log it out if the
//! redis data doesn't match the DB data, treating the DB data as accurate and
//! discarding the redis data if so. Here's how we can use an `Experiment` to do
//! this.
//!
//! ```
//! use thesis::{Experiment, rollout::Percent};
//!
//! async fn load_data_from_db(id: i32) -> i32 { id }
//! async fn load_data_from_redis(id: i32) -> i32 { id }
//!
//! # tokio_test::block_on(async {
//! let id = 4;
//! let result = Experiment::new("redis migration")
//!     .control(load_data_from_db(id))
//!     .experimental(load_data_from_redis(id))
//!     .rollout_strategy(Percent::new(0.5))
//!     .on_mismatch(|mismatch| {
//!         eprintln!(
//!             "DB & Redis data differ - db={}, redis={}",
//!             mismatch.control,
//!             mismatch.experimental,
//!         );
//!
//!         // the `control` value here comes from the DB
//!         mismatch.control
//!     })
//!     .run()
//!     .await;
//!
//! assert_eq!(result, 4);
//! # });
//! ```

pub mod experiment;
pub mod rollout;

pub use experiment::Experiment;
pub use rollout::{RolloutDecision, RolloutStrategy};

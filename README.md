# Scientist

Inspired by https://github.com/github/scientist

Scientist provides the `Experiment` struct, which represents an experiment to
run which compares the return values of multiple methods for accomplishing
the same task.

Let's imagine that we already have a function called `load_data_from_db`,
which loads some data from a database. We want to refactor this to instead
load the same data from redis. We write a new function called
`load_data_from_redis` to accomplish the same task, but with redis instead of
a DB. We want to use the redis version on only a very small percentage of
traffic, say 0.5% of incoming requests, and we want to log it out if the
redis data doesn't match the DB data, treating the DB data as accurate and
discarding the redis data if so. Here's how we can use an `Experiment` to do
this.

```rust
async fn load_data_from_db(id: i32) -> i32 { id }
async fn load_data_from_redis(id: i32) -> i32 { id }

let id = 4;
use scientist::Experiment;
let result = Experiment::new("load_data_from_db => load_data_from_redis")
    .control(load_data_from_db(id))
    .experimental(load_data_from_redis(id))
    .rollout_strategy(0.005)
    .on_mismatch(|mismatch| {
        eprintln!(
            "DB & Redis data differ - db={}, redis={}",
            mismatch.control,
            mismatch.experimental,
        );

        // the `control` value here comes from the DB
        mismatch.control
    })
    .run()
    .await;

assert_eq!(result, 4);
```

# Limitations

- The `control` and `experimental` futures must both have the same `Output`
  types
- There are no defaults provided for `control`, `experimental`, or
  `rollout_strategy`, all of these methods must be called or the experiment
  will not compile.
- `control` and `experimental` must both be futures. A non-async version of
  `Experiment` could be written, but this library does not currently provide
  one.

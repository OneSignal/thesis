# 0.1.0

- Initial relese

# 0.2.0

- Deprecated `impl RolloutStrategy for f64` in favor of the `Percent` rollout strategy

# 0.3.0

- Introduce the `run_result` method with special handling for `Result` values
- Remove `RolloutDecision::UseExperimental`
- Remove `thesis_experiment_run_mismatch` metric in favor of the more general
  `thesis_experiment_outcome`

# 0.4.0

- Upgrade to Tokio 1.0

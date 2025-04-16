use rand::Rng;

/// A decision of if the control or experimental methods should be used
#[derive(Clone, Copy)]
pub enum RolloutDecision {
    /// Run only the control method
    UseControl,

    /// Run both methods concurrently and compare the results. If the results do
    /// not match, the `on_mismatch` handler will be run.
    UseExperimentalAndCompare,

    // Run only the experimental method
    UseExperimental,
}

/// A method for chosing if the control or experimental code should run
pub trait RolloutStrategy {
    fn rollout_decision(&self) -> RolloutDecision;
}

impl RolloutStrategy for RolloutDecision {
    fn rollout_decision(&self) -> RolloutDecision {
        *self
    }
}

/// The simplest rollout strategy, a floating point number between 0 and 100 that
/// represents the percentage of requests which should use the experimental
/// method. The experimental results will be compared to the control results.
pub struct Percent(f64);

impl Percent {
    /// Create a new rollout Percent
    pub fn new(percent: f64) -> Self {
        Self(percent / 100.0)
    }
}

impl RolloutStrategy for Percent {
    fn rollout_decision(&self) -> RolloutDecision {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < self.0 {
            RolloutDecision::UseExperimentalAndCompare
        } else {
            RolloutDecision::UseControl
        }
    }
}

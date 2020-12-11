use rand::Rng;

/// A decision of if the control or experimental methods should be used
pub enum RolloutDecision {
    /// Run only the control method
    UseControl,

    /// Run only the experimental method
    UseExperimental,

    /// Run both methods concurrently and compare the results. If the results do
    /// not match, the `on_mismatch` handler will be run.
    UseExperimentalAndCompare,
}

/// A method for chosing if the control or experimental code should run
pub trait RolloutStrategy {
    fn rollout_decision(&self) -> RolloutDecision;
}

/// The simplest rollout strategy, a floating point number between 0 and 1 that
/// represents the percentage of requests which should use the experimental
/// method. The experimental results will be compared to the control results.
pub struct Ratio(f64);

impl Ratio {
    /// Create a new rollout Ratio
    pub fn new(ratio: f64) -> Self {
        Self(ratio)
    }
}

impl RolloutStrategy for Ratio {
    fn rollout_decision(&self) -> RolloutDecision {
        let mut rng = rand::thread_rng();

        if rng.gen::<f64>() < self.0 {
            RolloutDecision::UseExperimentalAndCompare
        } else {
            RolloutDecision::UseControl
        }
    }
}

//! LED controller — current-state manager.
//!
//! Receives LED commands (from Server via `NodeLedCommand` or from the local
//! offline state machine) and dispatches them to the LED driver. In M0 the
//! driver is a mock; the real USB HID driver for the ReSpeaker arrives at M6.
//!
//! **Invariant (Architecture §16.5):** Only animation ids are accepted.
//! Raw frame data from the network is rejected.

use crate::led::animations::{self, AnimationDef};
use std::sync::{Arc, Mutex};
use tracing;

/// Trait for LED drivers (real or mock).
pub trait LedDriver: Send + Sync {
    /// Set the current LED animation by its definition.
    fn set_animation(&self, animation: &AnimationDef);

    /// Turn off all LEDs.
    fn off(&self);
}

/// Mock LED driver that records commands for testing.
#[derive(Debug, Default)]
pub struct MockLedDriver {
    /// Recorded animation ids in order.
    pub commands: Arc<Mutex<Vec<String>>>,
}

impl MockLedDriver {
    /// Create a new mock driver.
    pub fn new() -> Self {
        Self {
            commands: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Get the list of recorded animation ids.
    pub fn recorded_commands(&self) -> Vec<String> {
        self.commands.lock().unwrap().clone()
    }
}

impl LedDriver for MockLedDriver {
    fn set_animation(&self, animation: &AnimationDef) {
        self.commands.lock().unwrap().push(animation.id.to_string());
    }

    fn off(&self) {
        self.commands.lock().unwrap().push("off".to_string());
    }
}

/// LED controller that manages the current LED state.
pub struct LedController {
    driver: Box<dyn LedDriver>,
}

impl LedController {
    /// Create a new controller with the given driver.
    pub fn new(driver: Box<dyn LedDriver>) -> Self {
        Self { driver }
    }

    /// Handle a `NodeLedCommand` by animation id.
    ///
    /// Looks up the animation in the local library. If found, dispatches to
    /// the driver. If not found, logs a warning and drops the command.
    /// This enforces the invariant that only known animation ids are accepted.
    pub fn handle_command(&self, animation_id: &str) {
        match animations::find_animation(animation_id) {
            Some(anim) => {
                tracing::debug!(animation_id = %animation_id, "Setting LED animation");
                self.driver.set_animation(anim);
            }
            None => {
                tracing::warn!(
                    animation_id = %animation_id,
                    "Unknown animation id — command dropped"
                );
            }
        }
    }

    /// Turn off all LEDs.
    pub fn off(&self) {
        self.driver.off();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mock_driver_records_commands() {
        let driver = MockLedDriver::new();
        let anim = animations::IDLE_BREATHE;
        driver.set_animation(&anim);
        assert_eq!(driver.recorded_commands(), vec!["idle-breathe"]);
    }

    #[test]
    fn controller_handles_known_animation() {
        let driver = MockLedDriver::new();
        let commands = driver.commands.clone();
        let controller = LedController::new(Box::new(driver));
        controller.handle_command("idle-breathe");
        assert_eq!(commands.lock().unwrap().as_slice(), &["idle-breathe"]);
    }

    #[test]
    fn controller_drops_unknown_animation() {
        let driver = MockLedDriver::new();
        let commands = driver.commands.clone();
        let controller = LedController::new(Box::new(driver));
        controller.handle_command("nonexistent-pattern");
        assert!(commands.lock().unwrap().is_empty());
    }

    #[test]
    fn controller_off() {
        let driver = MockLedDriver::new();
        let commands = driver.commands.clone();
        let controller = LedController::new(Box::new(driver));
        controller.off();
        assert_eq!(commands.lock().unwrap().as_slice(), &["off"]);
    }

    #[test]
    fn mock_driver_default() {
        let driver = MockLedDriver::default();
        assert!(driver.recorded_commands().is_empty());
    }
}

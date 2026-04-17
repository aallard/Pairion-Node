//! LED animation library for Pairion Node.
//!
//! Every animation from Architecture §7.1 is defined as a named constant with
//! metadata. The Node renders animations locally; the Server commands LED state
//! by referencing animation ids only.
//!
//! **Invariant (Architecture §16.5):** LED state is driven from the local
//! authoritative animation library. Network-delivered animation data is
//! rejected; only animation ids are accepted.

use serde::{Deserialize, Serialize};

/// Metadata for an LED animation pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationDef {
    /// Unique animation identifier referenced in `NodeLedCommand`.
    pub id: &'static str,
    /// Primary color as hex string (e.g., "#FFB347").
    pub color: &'static str,
    /// Duration in milliseconds (0 = continuous).
    pub duration_ms: u32,
    /// Human-readable pattern description.
    pub pattern: &'static str,
}

/// Idle breathing amber.
pub const IDLE_BREATHE: AnimationDef = AnimationDef {
    id: "idle-breathe",
    color: "#FFB347",
    duration_ms: 0,
    pattern: "Slow breathe at 5% brightness",
};

/// Wake word detected — amber spiral-in.
pub const WAKING_SPIRAL: AnimationDef = AnimationDef {
    id: "waking-spiral",
    color: "#FFB347",
    duration_ms: 400,
    pattern: "One-shot 400ms amber spiral-in",
};

/// Listening to an identified User — steady amber pulse.
pub const LISTENING_IDENTIFIED: AnimationDef = AnimationDef {
    id: "listening-identified",
    color: "#FFB347",
    duration_ms: 0,
    pattern: "Steady amber pulse to speech rhythm",
};

/// Listening to a Guest — steady cool-blue pulse.
pub const LISTENING_GUEST: AnimationDef = AnimationDef {
    id: "listening-guest",
    color: "#5B9BD5",
    duration_ms: 0,
    pattern: "Steady cool-blue pulse",
};

/// Voice-ID in progress — white rotating segment.
pub const IDENTIFYING_ROTATING: AnimationDef = AnimationDef {
    id: "identifying-rotating",
    color: "#FFFFFF",
    duration_ms: 600,
    pattern: "Fast rotation, 600ms",
};

/// Agent processing — amber arc-reactor pulse.
pub const THINKING_PULSE: AnimationDef = AnimationDef {
    id: "thinking-pulse",
    color: "#FFB347",
    duration_ms: 0,
    pattern: "Pulse 1 Hz",
};

/// Node outputting audio — amplitude-reactive cyan bars.
pub const SPEAKING_EQUALIZER: AnimationDef = AnimationDef {
    id: "speaking-equalizer",
    color: "#00CED1",
    duration_ms: 0,
    pattern: "Syncs to TTS amplitude",
};

/// Lost arbitration — purple sweep.
pub const HANDOFF_PURPLE_SWEEP: AnimationDef = AnimationDef {
    id: "handoff-purple-sweep",
    color: "#9370DB",
    duration_ms: 600,
    pattern: "One-shot purple sweep",
};

/// Approval pending — flashing yellow.
pub const APPROVAL_YELLOW_BLINK: AnimationDef = AnimationDef {
    id: "approval-yellow-blink",
    color: "#FFD700",
    duration_ms: 0,
    pattern: "2 Hz blink",
};

/// Smart Node offline — slow rotating muted teal.
pub const OFFLINE_SMART_TEAL: AnimationDef = AnimationDef {
    id: "offline-smart-teal",
    color: "#2F8B8B",
    duration_ms: 0,
    pattern: "Slow rotating segment",
};

/// Dumb Node offline — solid dimmed red.
pub const OFFLINE_DUMB_RED: AnimationDef = AnimationDef {
    id: "offline-dumb-red",
    color: "#8B0000",
    duration_ms: 0,
    pattern: "Solid dimmed red",
};

/// Reconnecting — teal to amber fade.
pub const RECONNECTING_TEAL_AMBER_FADE: AnimationDef = AnimationDef {
    id: "reconnecting-teal-amber-fade",
    color: "#2F8B8B",
    duration_ms: 0,
    pattern: "Single sweep teal to amber",
};

/// Error state — fast red pulse.
pub const ERROR_RED_PULSE: AnimationDef = AnimationDef {
    id: "error-red-pulse",
    color: "#FF0000",
    duration_ms: 0,
    pattern: "Fast pulse",
};

/// Quiet hours — all LEDs off.
pub const QUIET_HOURS_OFF: AnimationDef = AnimationDef {
    id: "quiet-hours-off",
    color: "#000000",
    duration_ms: 0,
    pattern: "All LEDs off",
};

/// All predefined animations.
pub const ALL_ANIMATIONS: &[&AnimationDef] = &[
    &IDLE_BREATHE,
    &WAKING_SPIRAL,
    &LISTENING_IDENTIFIED,
    &LISTENING_GUEST,
    &IDENTIFYING_ROTATING,
    &THINKING_PULSE,
    &SPEAKING_EQUALIZER,
    &HANDOFF_PURPLE_SWEEP,
    &APPROVAL_YELLOW_BLINK,
    &OFFLINE_SMART_TEAL,
    &OFFLINE_DUMB_RED,
    &RECONNECTING_TEAL_AMBER_FADE,
    &ERROR_RED_PULSE,
    &QUIET_HOURS_OFF,
];

/// Look up an animation by its id.
pub fn find_animation(id: &str) -> Option<&'static AnimationDef> {
    ALL_ANIMATIONS.iter().find(|a| a.id == id).copied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_animations_have_unique_ids() {
        let mut ids: Vec<&str> = ALL_ANIMATIONS.iter().map(|a| a.id).collect();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), ALL_ANIMATIONS.len());
    }

    #[test]
    fn find_existing_animation() {
        let anim = find_animation("idle-breathe");
        assert!(anim.is_some());
        assert_eq!(anim.unwrap().id, "idle-breathe");
    }

    #[test]
    fn find_nonexistent_animation() {
        assert!(find_animation("does-not-exist").is_none());
    }

    #[test]
    fn all_animations_have_color() {
        for anim in ALL_ANIMATIONS {
            assert!(anim.color.starts_with('#'));
        }
    }

    #[test]
    fn animation_count_matches_spec() {
        // Architecture §7.1 defines 14 animations
        assert_eq!(ALL_ANIMATIONS.len(), 14);
    }
}

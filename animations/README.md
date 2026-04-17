# Animations

This directory holds LED animation definitions for the Node's ReSpeaker
USB mic array (12 APA102 LEDs).

In M0, animations are defined as constants in `src/led/animations.rs`.
At M6, this directory may hold additional animation data files (timing
curves, color palettes) that the LED controller loads at startup.

See `Pairion-Node-Architecture.md` §7.1 for the full animation catalog.

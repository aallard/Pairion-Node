# Models

This directory holds wake-word and VAD models shipped with the Node binary.

In M0, no model files are present. At M6:
- `hey-pairion.onnx` — openWakeWord model for "Hey, Pairion" detection
- `silero-vad.onnx` — Silero VAD model for speech endpoint detection

Smart-tier models (Whisper-small, mini-LLM) are downloaded at pairing time
from the Server and stored under `/opt/pairion-node/models/` on the Pi,
not bundled here.

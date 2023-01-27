# Version History
## v0.7.0
### Features
- Add `--continue` flag, to continue timer after it finishes.

### Fixes
- Timer now uses `SystemTime` to update time, to avoid pausing timer when  
  computer goes to sleep. Use `Instant` as fallback if `SystemTime` jumps back in time.

### Changes
- `TimerState` implements `PartialEq` and `Eq`.

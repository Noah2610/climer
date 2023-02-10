# Version History
## v0.7.1 - 2023-02-10
### Fixes
- Timer now appropriately continues into negative time when computer
  goes to sleep while target time is reached with `-c` flag.

## v0.7.0 - 2023-01-27
### Features
- Add `--continue` flag, to continue timer after it finishes.

### Fixes
- Timer now uses `SystemTime` to update time, to avoid pausing timer when  
  computer goes to sleep. Use `Instant` as fallback if `SystemTime` jumps back in time.

### Changes
- `TimerState` implements `PartialEq` and `Eq`.

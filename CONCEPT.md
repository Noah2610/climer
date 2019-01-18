# Climer Concept
Should be a simple and easy to use command-line timer.

## Examples

```
# Timer for 10 minutes
climer 10M
# Timer for 10 minutes and 20 seconds
climer 10M20S
climer 20S 10M
climer 10:20
climer 10.20
climer 20:10 -f %S.%M
climer "20   10" -f %S\s*%M
# Execute when done - UNNECESSARY, just use `climer 10s && notify-send "Time's Up!"`
climer 10:20 -c "notify-send 'Time\'s Up!'"
# Custom output format
climer 10:20 -o "<%M-%S-%N>"
# Quiet, no output
climer 10s -q
```

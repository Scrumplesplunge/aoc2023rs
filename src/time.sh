#!/bin/bash

# cpupower is required for putting the CPU into performance mode.
which cpupower || exit 1
# microtime is github.com/scrumplesplunge/microtime, and is required for timing.
which microtime || exit 1

sudo cpupower frequency-set --governor performance
for day in day{01..25}; do
  2>&1 printf "$day: "
  microtime "inputs/$day.input" "target/release/$day" >/dev/null
done
sudo cpupower frequency-set --governor powersave

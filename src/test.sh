#!/bin/bash

for output in inputs/day*.output; do
  day="$(basename -s '.output' "$output")"
  if ! [[ -f "target/debug/$day" ]]; then
    echo "$day: no solver"
    continue
  fi
  if ! [[ -f "target/$day.diff" ]] ||
     ! [[ "target/$day.diff" -nt "target/debug/$day" ]]; then
    if ! "target/debug/$day" <"inputs/$day.input"  \
                             >"target/$day.output.tmp"; then
      echo -e "$day: \x1b[31mexited unsuccessfully\x1b[0m"
      continue
    fi
    mv "target/$day.output"{.tmp,}
  fi
  if diff --color=always "$output" "target/$day.output"  \
          >"target/$day.diff"; then
    echo -e "$day: \x1b[32mPASSED\x1b[0m"
  else
    echo -e "$day: \x1b[31mFAILED\x1b[0m"
    cat "target/$day.diff"
  fi
done

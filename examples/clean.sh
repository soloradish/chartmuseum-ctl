#!/bin/bash -ex

# export CHARTMUSEUM_URL=http://127.0.0.1

# list all charts
cs=$(cm-cli list)

for c in $cs
do
  echo "Start clean ${c}"
  # Delete charts craeted before 50 days.
  cm-cli versions $c --before 50 | xargs -tL 1 cm-cli delete $c
done

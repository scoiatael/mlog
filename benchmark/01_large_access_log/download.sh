#!/usr/bin/env bash

SELF="$(dirname $0)"
FILE="$SELF/sample.txt"

test -f $FILE || wget https://raw.githubusercontent.com/CharlyWargnier/Server_Log_Analyser_for_SEO/23166cbb9a5d3b6692c2e07a90703e6ce39fe6d3/Sample_CSV_Files/Loggy17Oct16.csv -O "$FILE"
for S in 100 200 300 400; do
  SMALL_FILE="${FILE}_$S"
  test -f $SMALL_FILE || (head -n $S $FILE > "$SMALL_FILE")
done


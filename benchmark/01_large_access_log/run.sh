#!/usr/bin/env bash

F=(sample.txt_*)
FILES=$(IFS=','; echo  "${F[*]}")

hyperfine --warmup 1 -L file "$FILES" 'bash use_mlog.sh {file}' 'bash use_grep.sh {file}'

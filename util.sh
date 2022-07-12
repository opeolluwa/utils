#!/bin/bash/
# AN UTILITY SCRIPT TO WATCH TYPESCRIPT AND sass 
# Author : Adeoye Adefemi Opeoluwa
# Last Modified : July (9, 2022)
#-----------------------------------------
#https://stackoverflow.com/questions/10909685/run-parallel-multiple-commands-at-once-in-the-same-terminal
#wrap the sass compiler in a variable (sass) same for typescript (TS)
for cmd in "$@"; do {
  echo "Process \"$cmd\" started";
  $cmd & pid=$!
  PID_LIST+=" $pid";
} done

trap "kill $PID_LIST" SIGINT

echo "Parallel processes have started";

wait $PID_LIST

echo
echo "All processes have completed";


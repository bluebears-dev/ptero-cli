#!/usr/bin/env bash

SIMULTANEOUS_JOBS=10
START=$1
END=$2
MEASUREMENT_EXECS=100
DATA_SIZE=30
# Measure bitrates for pivots in START..END range 
# Execute commands in parallel which calculates average bitrate for 100 executions
# Get the average by `tail -1`
seq $START $END | parallel -j ${SIMULTANEOUS_JOBS} --workdir $PWD -- "./scripts/bitrate_for_pivot.sh {} $MEASUREMENT_EXECS $DATA_SIZE | tail -1"
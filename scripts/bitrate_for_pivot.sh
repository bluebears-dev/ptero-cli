#!/usr/bin/env bash

PIVOT=$1
END=$2
DATA_SIZE_BYTES=$3

UUID=`uuidgen`
BITRATE_FILE="$(pwd)/bitrates-${UUID}"
ENCODED_FILE="$(pwd)/encoded-${UUID}"
COVER_FILE="$(pwd)/resources/cover/cover_utf8.txt"
DATA_FILE="$(pwd)/data-${UUID}"
METHOD="eluv"
PTERO_CLI="$(pwd)/target/debug/ptero_cli"

# Ensure there are no leftovers
rm -f ${BITRATE_FILE} ${ENCODED_FILE} ${DATA_FILE} &> /dev/null

echo "Measuring bitrate $END times..."
for I in $(seq 1 ${END}); do
    ## Uncomment for max capacity testing
    # DATA_SIZE_BITS=`${PTERO_CLI} capacity -c ${COVER_FILE} --pivot ${PIVOT} --${METHOD} | tail -n 1 | cut -f1 -d" "`
    # DATA_SIZE_BYTES=$(expr ${DATA_SIZE_BITS} / 8)
    head -c ${DATA_SIZE_BYTES} </dev/urandom >${DATA_FILE}

    CMD="${PTERO_CLI} -o ${ENCODED_FILE} encode -c ${COVER_FILE} -d ${DATA_FILE} --pivot ${PIVOT} --${METHOD}"
    bash -c "${CMD}" || { echo -e "#\c"; continue; }

    DATA_FILE_SIZE=`du -b ${DATA_FILE} | cut -f1`
    ENCODED_FILE_SIZE=`du -b ${ENCODED_FILE} | cut -f1`
    BITRATE=`echo ${DATA_FILE_SIZE} / ${ENCODED_FILE_SIZE} | bc -l`
    # Print without newline
    echo -e ".\c"
    echo $BITRATE >> ${BITRATE_FILE}
done
# Start with newline after finishing 'progress bar'
echo ""
if [ -f "$BITRATE_FILE" ]; then
    echo "Calculating average bitrate..."
    AVERAGE_BITRATE=`awk '{ total += $1; count++ } END { print total/count }' ${BITRATE_FILE}`
    echo "$PIVOT    $AVERAGE_BITRATE"
else
    echo "$PIVOT    ERROR"
fi

# Clean-up after measurment
rm ${BITRATE_FILE} ${ENCODED_FILE} ${DATA_FILE} &> /dev/null
#!/usr/bin/env bash

PIVOT=$1
END=$2
DATA_SIZE_BYTES=$3

UUID=`uuidgen`
BITRATE_FILE="bitrates-${UUID}"
ENCODED_FILE="encoded-${UUID}"
COVER_FILE="resources/cover/cover_utf8.txt"
DATA_FILE="data-${UUID}"

# Ensure there are no leftovers
rm -f ${BITRATE_FILE} ${ENCODED_FILE} ${DATA_FILE} &> /dev/null

echo "Measuring bitrate $END times..."
for I in $(seq 1 ${END}); do
    head -c ${DATA_SIZE_BYTES} </dev/urandom >${DATA_FILE}
    CMD="cargo run -- -o ${ENCODED_FILE} encode -c ${COVER_FILE} -d ${DATA_FILE} --pivot ${PIVOT}" 

    bash -c "${CMD}" &>/dev/null || { echo -e "#\c"; continue; }
    COVER_FILE_SIZE=`du -b ${COVER_FILE} | cut -f1`
    ENCODED_FILE_SIZE=`du -b ${ENCODED_FILE} | cut -f1`
    BITRATE=`echo ${COVER_FILE_SIZE} / ${ENCODED_FILE_SIZE} | bc -l`
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
#!/bin/bash

# Set the directory containing the images
DIR=/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master

# Get the number of images in the directory
RANGE=$(ls -1 "$DIR"/*.jpg | wc -l)

# Set the minimum and maximum wait time in seconds
MIN_WAIT=60
MAX_WAIT=120

while true; do
    # Generate a random number between 1 and $RANGE
    number=$(openssl rand -hex 4 | awk '{print strtonum("0x"substr($0,0,4))}')
    let "number %= $RANGE"

    # Set the wallpaper to the selected image
    COUNTER=1
    for X in "$DIR"/*.jpg; do
        if [ $number -eq $COUNTER ]; then
            # Use a different command to set the wallpaper if pcmanfm is not available
            pcmanfm --set-wallpaper "$X"
        fi
        let "COUNTER++"
    done

    # Wait for a random amount of time between $MIN_WAIT and $MAX_WAIT seconds
    sleep $(shuf -i $MIN_WAIT-$MAX_WAIT -n 1)
done

#!/bin/bash

# Get the directory of photos to use for the wallpaper
# wallpaper_dir="/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master"

#!/bin/sh
while true;
do
   find /media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master -type f \( -name '*.jpg' -o -name '*.png' \) -print0 | shuf -n 1 -z | xargs -0 pcmanfm --wallpaper-mode=1
   sleep 2m
done &
#!/bin/bash

# Get the directory of photos to use for the wallpaper
wallpaper_dir="/media/pipi/e9535df1-d952-4d78-b5d7-b82e9aa3a975/Master"

# Set the interval in minutes between wallpaper rotations
interval=2

# Get a list of all the images in the wallpaper directory
images=$(ls "$wallpaper_dir"/*.jpg)

# Choose a random image from the list
random_image=$(shuf -n 1 <<< "$images")

# Set the wallpaper to the chosen image
lxc config set desktop wallpaper "$random_image"

# Start a loop to rotate the wallpaper every 2 minutes
while true; do
  sleep "$interval"m
  random_image=$(shuf -n 1 <<< "$images")
  lxc config set desktop wallpaper "$random_image"
done
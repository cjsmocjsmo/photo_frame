#!/bin/bash

cd "/media/pipi/0123-4567/Images";

find . -type f -name "*.gz" | while read file; do
  echo "working on $file"
  gunzip "$file"
  rm "$file"
  echo "removed $file"
done
find . -type f -name "*.GZ" | while read file; do
  echo "working on $file"
  gunzip "$file"
  rm "$file"
  echo "removed $file"
done
find . -type f -name "*.zip" | while read file; do
  echo "working on $file"
  unzip "$file"
  rm "$file"
  echo "removed $file"
done
find . -type f -name "*.ZIP" | while read file; do
  echo "working on $file"
  unzip "$file"
  rm "$file"
  echo "removed $file"
done
find . -type f -name "*.bz2" | while read file; do
  echo "working on $file"
  bunzip2 "$file"
  rm "$file"
  echo "removed $file"
done
find . -type f -name "*.BZ2" | while read file; do
  echo "working on $file"
  bunzip2 "$file"
  rm "$file"
  echo "removed $file"
done
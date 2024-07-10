#!/bin/bash

# Loop melalui setiap direktori di level pertama
for dir in ./*; do

  # Pastikan ini memang direktori dan bukan file biasa
  if [[ -d "$dir" ]]; then

    # Periksa keberadaan folder .git di dalam direktori
    if [[ -d "$dir/.git" ]]; then
      
      # Hapus folder .git jika ada
      echo "Menghapus .git di $dir"
      rm -rf "$dir/.git"
    fi
  fi

done

#!/bin/bash
while IFS= read -r file; do 
echo "${file##*.}"; 
done < <(find linux-6.4 -name '*.*') \
| sort \
| uniq -c \
| sort -nr \
| head -n 8
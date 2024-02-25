#!/bin/bash

check_helper() {
  # initializes variable
  local dir=$1

# results from find (after done) are given to the if statement a file input
  while IFS= read -r -d '' file; do 
    if [[ -f "${file}" ]]; then  
    # increases total count by 1 for each .sh file
      total_count=$((total_count + 1))
      if shellcheck "${file}" >/dev/null; then  
        pass_count=$((pass_count + 1))  #  increases the passcount by 1 if the file passes 
      else 
        error_found=true
      fi
    fi
  done < <(find "${dir}" -name '*.sh' -print0)
}

# if no argument is given, the directory equals the current directory
if [ $# -gt 1 ]; then 
  echo "Usage: Too many arguments" >&2 # prints to stderr
  exit 1

# if there is an argument, the directory equals to the argument's directory
elif [ $# -eq 1 ]; then 
  dir="$1"
  # if the directory does not exist, we write an error to stderr and exit with 1.
  if [ ! -d "${dir}" ]; then
  echo "Error: ${dir} is not a directory" >&2 # prints to stderr
  exit 1
  fi
# if more than two arguments are given, we echo the usage to stderr and exit with 1.
else
  dir="./"
fi

# initializes counting variables
total_count=0  
pass_count=0
error_found=false

check_helper "${dir}"
echo "$pass_count out of $total_count shell scripts passed shellcheck"

#  exits if the error is found
if error_found=true; then 
exit 1
fi
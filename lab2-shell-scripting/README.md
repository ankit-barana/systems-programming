Ankit Barana - I have adhered to the honor code in this assignment
Sean Norton - I have adhered to the honor code in this assignment

**Part 1** - The five complex number values:
-0.4 + 0.65i
-0.45 + 0.6i
-0.45 + 0.55i
-0.45 + 0.575i
-0.5 + 0.575i

Code:
for complex_num in "-0.4 + 0.65i" "-0.45 + 0.6i" "-0.45 + 0.55i" "-0.45 + 0.575i" "-0.5 + 0.575i"; do
    for size in "200" "400" "800"; do
        cargo run -- --constant="$complex_num" --size="$size" "Julia Set ${complex_num} ${size}x${size}.png"
    done
done


**Part 2** - Simple Pipeline  
curl -L https://github.com/torvalds/linux/archive/refs/tags/v6.4.tar.gz | tar -zx -f -


**Part 3** 
PLEASE READ!!! To prevent the shellcheck output including warnings, errors, etc. from showing up in the terminal, I used >/dev/null. It allows us to only see only how many of the .sh files passed shellcheck. Also for one of the exmaple commands - ./shellcheckall.sh linux-6.4/tools/perf - the output given in the instructions is "6 of 74 shell scripts passed shellcheck" but I got "3 of 74 shell scripts passed shellcheck". Otherwise, the output (the first three lines mentioned in the intructions) matches with what the intructions perfectly. Hence, I think that I might have been working with a different set if files(?)

Question - How many shell scripts in the entire linux-6.4 pass spellcheck?
Answer - 74 out of 814 shell scripts passed shellcheck

For loop:
for dir in linux-6.4/*/; do echo "$dir: $(./shellcheckall.sh "${dir}")"; done

Output: 
linux-6.4/Documentation/: 1 out of 9 shell scripts passed shellcheck
linux-6.4/LICENSES/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/arch/: 15 out of 39 shell scripts passed shellcheck
linux-6.4/block/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/certs/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/crypto/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/drivers/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/fs/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/include/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/init/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/io_uring/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/ipc/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/kernel/: 0 out of 1 shell scripts passed shellcheck
linux-6.4/lib/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/mm/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/net/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/rust/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/samples/: 1 out of 21 shell scripts passed shellcheck
linux-6.4/scripts/: 9 out of 51 shell scripts passed shellcheck
linux-6.4/security/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/sound/: 0 out of 0 shell scripts passed shellcheck
linux-6.4/tools/: 48 out of 692 shell scripts passed shellcheck
linux-6.4/usr/: 0 out of 1 shell scripts passed shellcheck
linux-6.4/virt/: 0 out of 0 shell scripts passed shellcheck



**Part 4** - Complex Pipeline
#!/bin/bash
while IFS= read -r file; do 
echo "${file##*.}"; 
done < <(find linux-6.4 -name '*.*') \
| sort \
| uniq -c \
| sort -nr \
| head -n 8

Output:
32459 c
23737 h
3488 yaml
3339 rst
2710 dts
2030 dtsi
1854 txt
1343 S
#!/bin/bash
declare -A ranges
while IFS= read -r line; do
    if [[ "$line" =~ ([0-9]+)-([0-9]+) ]]; then
        ranges[${BASH_REMATCH[1]},${BASH_REMATCH[2]}]="butthanks"
    fi
done < "$1"

current=(0 -1)
total=0
while IFS= read -r key; do
    start=$(echo $key | cut -d',' -f 1)
    end=$(echo $key | cut -d',' -f 2)
    if [[ "${current[1]}" -ge $start ]]; then
        start=${current[0]}
        if [[ "${current[1]}" -ge "$end" ]]; then
            end=${current[1]}
        fi
    else
        total=$(($total + ${current[1]} - ${current[0]} + 1))
    fi
    current=($start $end)
done < <(printf "%s\n" "${!ranges[@]}" | sort -t, -k1,1n)
total=$(($total + ${current[1]} - ${current[0]} + 1))
echo $total
#!/bin/bash

MAX_TEMP=70
CHECK_INTERVAL=0.3
THERMAL_ZONE="/sys/class/thermal/thermal_zone3/temp"
COOLDOWN_TEMP=$((MAX_TEMP - 20))  # 50°C in this case

[ ! -f "$THERMAL_ZONE" ] && pino -t "Error" -m "Thermal zone not found!" -d 10 && exit 1

while true; do
    temp=$(($(cat "$THERMAL_ZONE") / 1000))
    
    if [ "$temp" -gt "$MAX_TEMP" ]; then
        pino -t "CPU Overheating Warning" -m "Current temperature: ${temp}°C" -d 5
        while [ "$temp" -gt "$COOLDOWN_TEMP" ]; do
            sleep "$CHECK_INTERVAL"
            temp=$(($(cat "$THERMAL_ZONE") / 1000))
        done
    fi
    
    sleep "$CHECK_INTERVAL"
done

#!/bin/bash

UP_THRESHOLD=90
DOWN_THRESHOLD=20
CHECK_INTERVAL=0.3

BATTERY_PATH=$(find /sys/class/power_supply/ -name "BAT*" | head -1)
AC_PATH=$(find /sys/class/power_supply/ -name "AC*" | head -1)
[ -z "$BATTERY_PATH" ] && echo "No battery found" && exit 1

last_level=$(cat "$BATTERY_PATH/capacity")
last_ac_status=$(cat "$AC_PATH/online" 2>/dev/null || echo "0")

while true; do
    level=$(cat "$BATTERY_PATH/capacity")
    ac_status=$(cat "$AC_PATH/online" 2>/dev/null || echo "0")
    
    if [ "$ac_status" -eq 1 ] && [ "$last_ac_status" -eq 0 ]; then
        pino -t "Plugged In" -m "Now charging ($level%)" -d 5

    elif [ "$ac_status" -eq 0 ] && [ "$last_ac_status" -eq 1 ]; then
        pino -t "Unplugged" -m "On battery ($level%)" -d 5
    
    elif [ "$level" -ge $UP_THRESHOLD ] && [ "$last_level" -lt $UP_THRESHOLD ] && [ "$ac_status" -eq 0 ]; then
        pino -t "Battery High" -m "Reached $level%" -d 5
    
    elif [ "$level" -le $DOWN_THRESHOLD ] && [ "$last_level" -gt $DOWN_THRESHOLD ] && [ "$ac_status" -eq 0 ]; then
        pino -t "Battery Low" -m "Only $level% left" -d 5
    fi
    
    last_level=$level
    last_ac_status=$ac_status
    sleep $CHECK_INTERVAL
done

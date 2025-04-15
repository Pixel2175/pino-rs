#!/bin/bash

MEM_HIGH=85    
MEM_LOW=65     
CHECK_INTERVAL=0.5

alert_triggered=false

while true; do
    mem_used=$(free | awk '/Mem/{usage=$3/$2*100; printf("%.0f", usage)}')
    if [ "$mem_used" -ge "$MEM_HIGH" ] && [ "$alert_triggered" = false ]; then
        pino -t "Memory Alert" -m "High RAM usage: ${mem_used}%" -d 5
        alert_triggered=true
    elif [ "$mem_used" -lt "$MEM_LOW" ] && [ "$alert_triggered" = true ]; then
        alert_triggered=false
    fi
    
    sleep "$CHECK_INTERVAL"
done

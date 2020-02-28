#!/bin/bash

HOST = ${1:-"127.0.0.1"}

for CONCUR in 1 5 10 30 60 100
do
	wrk -t10 -c100 -d20s "http://$HOST:3002/tasks" &
	WRK = $!
	sleep 10
	./snapshot.sh
	wait $WRK
done


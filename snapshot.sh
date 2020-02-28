echo "postgres"
top -b -d 1 -n 1 | grep postgres | awk '{cpu += $9; rss += $10} END {print cpu/100, rss}'
echo "node"
top -b -d 1 -n 1 | grep node | awk '{cpu += $9; rss += $10} END {print cpu/100, rss}'
echo "actix"
top -b -d 1 -n 1 | grep actix | awk '{cpu += $9; rss += $10} END {print cpu/100, rss}'

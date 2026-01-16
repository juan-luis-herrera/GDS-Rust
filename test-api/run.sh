if [ -n "$NTFY_REPORT" ]; then
curl -d "Starting GDS" $NTFY_REPORT
fi
cargo run > /results/Results.csv
if [ -n "$NTFY_REPORT" ]; then
curl -d "GDS finished" $NTFY_REPORT
fi
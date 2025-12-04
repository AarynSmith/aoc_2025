for bin in $(cargo metadata --no-deps --format-version=1 \
    | jq -r '.packages[] | select(.name=="aoc_2024") | .targets[] | select(.kind[]=="bin") | .name'); do
    echo "==== Running $bin ===="
    cargo run --quiet --bin "$bin"
done
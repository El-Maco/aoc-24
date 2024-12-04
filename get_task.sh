#!/bin/bash

YEAR=2024
DAY=${1:-$(date +%-d)}
PADDED_DAY=$(printf "%02d" "$DAY")
AOC_SESSION=$(cat .aoc_session)
AOC_URL="https://adventofcode.com/$YEAR/day/$DAY"
CHALLENGES_DIR="src/challenges"

curl -H "Cookie: session=$AOC_SESSION" $AOC_URL -o "tasks/day${PADDED_DAY}.html"

curl -H "Cookie: session=$AOC_SESSION" $AOC_URL/input -o "inputs/day${PADDED_DAY}.txt"

echo "Fetched AoC Day $DAY tasks and input."

RS_FILE="$CHALLENGES_DIR/day$PADDED_DAY.rs"
MOD_FILE="$CHALLENGES_DIR/mod.rs"

[ -f $RS_FILE ] || cp "$CHALLENGES_DIR/template.rs" "$RS_FILE"
sed -i "s/load_input(.*)/load_input($DAY)/" "$RS_FILE"
 if [[ ! $(grep "day$PADDED_DAY" $MOD_FILE) ]];then
     echo "pub mod day$PADDED_DAY;" >> $MOD_FILE
 fi


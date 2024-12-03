#!/bin/bash

YEAR=2024
DAY=${1:-$(date +%-d)}
PADDED_DAY=$(printf "%02d" "$DAY")
AOC_SESSION=$(cat .aoc_session)
AOC_URL="https://adventofcode.com/$YEAR/day/$DAY"

curl -H "Cookie: session=$AOC_SESSION" $AOC_URL -o "tasks/day${PADDED_DAY}.html"

curl -H "Cookie: session=$AOC_SESSION" $AOC_URL/input -o "inputs/day${PADDED_DAY}.txt"

echo "Fetched AoC Day $DAY tasks and input."

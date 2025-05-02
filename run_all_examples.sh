#!/bin/bash

# Create output directory if it doesn't exist
mkdir -p output

# ANSI color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}Running all mesh-tools examples...${NC}\n"

# Get list of all example files directly
examples=$(ls examples | grep -E "^[0-9][0-9]_[a-z_]*.rs$" | sed 's/\.rs$//')

# Check if user specified certain examples to run
if [ $# -gt 0 ]; then
    # Use examples specified by user
    examples="$@"
fi

# Run each example
for example in $examples; do
    echo -e "${GREEN}Running example: ${example}${NC}"
    cargo run --example "$example"
    echo -e "\n"
done

echo -e "${BLUE}All examples completed. Results are in the 'output' directory.${NC}"
echo -e "${BLUE}Examples generated:${NC}"
ls -lh output/

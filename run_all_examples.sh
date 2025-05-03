#!/bin/bash

# Create output directory if it doesn't exist
mkdir -p output

# ANSI color codes
GREEN='\033[0;32m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${BLUE}Running all mesh-tools examples...${NC}\n"

# Get list of all example files - get all .rs files in the examples directory
examples=$(find examples -maxdepth 1 -name "*.rs" | sed 's|examples/||' | sed 's/\.rs$//')

# Check if user specified certain examples to run
if [ $# -gt 0 ]; then
    # Use examples specified by user
    examples="$@"
fi

# Run each example
success_count=0
fail_count=0
failed_examples=""

for example in $examples; do
    echo -e "${GREEN}Running example: ${example}${NC}"
    if cargo run --example "$example"; then
        echo -e "${GREEN}✓ Example $example completed successfully${NC}\n"
        success_count=$((success_count + 1))
    else
        echo -e "${RED}✗ Example $example failed${NC}\n"
        fail_count=$((fail_count + 1))
        failed_examples="$failed_examples $example"
    fi
done

echo -e "${BLUE}All examples completed. Results are in the 'output' directory.${NC}"
echo -e "${GREEN}Successfully ran: $success_count examples${NC}"

if [ $fail_count -gt 0 ]; then
    echo -e "${RED}Failed: $fail_count examples${NC}"
    echo -e "${RED}Failed examples:${failed_examples}${NC}"
fi

echo -e "${BLUE}Examples generated:${NC}"
ls -lh output/ | sort

#!/usr/bin/env bash

###############################################################################
#
# Dependencies:
#
# $ cargo install cargo-tarpaulin
#
###############################################################################

WORKING_DIRECTORY=$(pwd)

cargo +stable tarpaulin --workspace --all-features --force-clean --out Html --engine llvm --output-dir "$WORKING_DIRECTORY/target/tov"

echo ""
echo "Open the coverage report:"
echo "  HTML file://$WORKING_DIRECTORY/target/tov/tarpaulin-report.html"
echo ""

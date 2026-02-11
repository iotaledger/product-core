#!/bin/bash

set -e

echo "Deploying IOTA Move contracts..."

# Deploy child contracts first
echo "Building and deploying child contracts in TfComponents package ..."
cd ../components_move/
iota move build
echo "Publishing TfComponents package..."
TF_COMP_RESULT=$(iota client publish --gas-budget 100000000 . --json)
TF_COMP_PACKAGE=$(echo "$TF_COMP_RESULT" | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
echo "TfComponents Package ID: $TF_COMP_PACKAGE"
cd ../borrow_non_store_obj_poc

echo ""

# Deploy parent contract (depends on child)
echo "Building and deploying parent contract..."
iota move build
echo "Publishing parent contract..."
PARENT_RESULT=$(iota client publish --gas-budget 100000000 . --json)
PARENT_PACKAGE=$(echo "$PARENT_RESULT" | jq -r '.objectChanges[] | select(.type == "published") | .packageId')
echo "Parent Package ID: $PARENT_PACKAGE"
cd ..

echo ""
echo "Deployment complete!"
echo ""
echo "=== Contract Addresses ==="
echo "TF_COMP_PACKAGE=\"$TF_COMP_PACKAGE\""
echo "PARENT_PACKAGE=\"$PARENT_PACKAGE\""
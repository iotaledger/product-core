#!/bin/bash

set -e

echo "PTB POC: Transfer Child to Parent"

# You need to manually set these package IDs after running deploy.sh
TF_COMP_PACKAGE="0x2acbe845269d520a5a4ea272041d19be5e561fd21df2f256c807c7bac769e987"
PARENT_PACKAGE="0x45bcf3c2364cef272628362e87ed9e897ef8d11dfad9608cc6559fbf4cda2f2f"

# Check if package IDs are set
if [ -z "$TF_COMP_PACKAGE" ] || [ -z "$PARENT_PACKAGE" ]; then
    echo "Error: Please update CHILD_PACKAGE and PARENT_PACKAGE variables with actual package IDs from deploy.sh"
    exit
fi

echo "TfComponents Package: $TF_COMP_PACKAGE"
echo "Parent Package: $PARENT_PACKAGE"
echo ""

# Step 1: Create child object
echo "Creating child object..."
CHILD_RESULT=$(iota client ptb --move-call $TF_COMP_PACKAGE::example_child::create --gas-budget 100000000 --json)
CHILD_OBJECT_ID=@$(echo "$CHILD_RESULT" | jq -r '.objectChanges[] | select(.type == "created") | .objectId' | head -1)
echo "Child Object ID: $CHILD_OBJECT_ID"

echo ""

# Step 2: Create parent object
echo "Creating parent object..."
PARENT_RESULT=$(iota client ptb --move-call $PARENT_PACKAGE::parent::create --gas-budget 100000000 --json)
PARENT_OBJECT_ID=@$(echo "$PARENT_RESULT" | jq -r '.objectChanges[] | select(.type == "created") | .objectId' | head -1)

echo "Parent Object ID: $PARENT_OBJECT_ID"

echo ""

# Step 3: Transfer child to parent object (using @ for address conversion)
echo "Transferring child object to parent..."
CHANGES=$(iota client ptb \
--assign child_object_id $CHILD_OBJECT_ID \
--assign parent_object_id $PARENT_OBJECT_ID \
--move-call $TF_COMP_PACKAGE::example_child::transfer_object  child_object_id parent_object_id \
--gas-budget 100000000)

echo ""

# Step 4: Borrow child from parent and increment counter
echo "Borrowing child from parent and incrementing..."
CHANGES=$(iota client ptb \
--assign child_object_id $CHILD_OBJECT_ID \
--assign parent_object_id $PARENT_OBJECT_ID \
--move-call $TF_COMP_PACKAGE::borrowed_child::request_example child_object_id \
--assign CHILD_REQUEST \
--move-call $PARENT_PACKAGE::parent::borrow_child parent_object_id CHILD_REQUEST \
--assign BORROWED_CHILD \
--move-call $TF_COMP_PACKAGE::borrowed_child::extract_example BORROWED_CHILD \
--assign PLEDGE_AND_CHILD \
--move-call $TF_COMP_PACKAGE::example_child::increment PLEDGE_AND_CHILD.1 \
--move-call $TF_COMP_PACKAGE::borrowed_child::example PLEDGE_AND_CHILD.0 PLEDGE_AND_CHILD.1 \
--assign BORROWED_CHILD \
--move-call $PARENT_PACKAGE::parent::put_back parent_object_id BORROWED_CHILD \
--gas-budget 100000000)

echo "Getting counter..."
CHANGES=$(iota client ptb \
--assign child_object_id $CHILD_OBJECT_ID \
--assign parent_object_id $PARENT_OBJECT_ID \
--move-call $TF_COMP_PACKAGE::borrowed_child::request_example child_object_id \
--assign CHILD_REQUEST \
--move-call $PARENT_PACKAGE::parent::borrow_child parent_object_id CHILD_REQUEST \
--assign BORROWED_CHILD \
--move-call $TF_COMP_PACKAGE::borrowed_child::extract_example BORROWED_CHILD \
--assign PLEDGE_AND_CHILD \
--move-call $TF_COMP_PACKAGE::example_child::get_counter PLEDGE_AND_CHILD.1 \
--move-call $TF_COMP_PACKAGE::borrowed_child::example PLEDGE_AND_CHILD.0 PLEDGE_AND_CHILD.1 \
--assign BORROWED_CHILD \
--move-call $PARENT_PACKAGE::parent::put_back parent_object_id BORROWED_CHILD \
--gas-budget 100000000)

echo "Counter: $CHANGES"
echo ""

echo ""
echo "POC Complete!"
echo ""
echo "=== Object IDs ==="
echo "Child:  $CHILD_OBJECT_ID"
echo "Parent: $PARENT_OBJECT_ID"
echo ""
echo "The child object has been successfully received by the parent object."
#!/bin/bash

# Copyright 2020-2026 IOTA Stiftung
# SPDX-License-Identifier: Apache-2.0

set -euo pipefail

if [[ "$#" -lt 1 || "$#" -gt 2 ]]; then
    echo "usage: $0 <chain-id> [alias]" >&2
    exit 1
fi

chain_id="$1"
alias="${2:-}"

script_dir=$(cd "$(dirname "$0")" && pwd)
package_dir="$script_dir/.."
history_file="$package_dir/Move.history.json"

response=$(iota client publish --silence-warnings --json --gas-budget 500000000 "$package_dir")
package_id=$(echo "$response" | jq -r '.objectChanges[] | select(.type | contains("published")) | .packageId')

tmp_file=$(mktemp)

if [[ -n "$alias" ]]; then
    jq \
        --arg chain_id "$chain_id" \
        --arg alias "$alias" \
        --arg package_id "$package_id" \
        '
        .aliases[$alias] = $chain_id
        | .envs[$chain_id] = ((.envs[$chain_id] // []) + [$package_id] | unique)
        ' \
        "$history_file" > "$tmp_file"
else
    jq \
        --arg chain_id "$chain_id" \
        --arg package_id "$package_id" \
        '
        .envs[$chain_id] = ((.envs[$chain_id] // []) + [$package_id] | unique)
        ' \
        "$history_file" > "$tmp_file"
fi

mv "$tmp_file" "$history_file"

echo "$package_id"

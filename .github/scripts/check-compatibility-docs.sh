#!/usr/bin/env bash
# Check that README.md documents the SDK/XDR versions pinned by the manifests.
# This complements check-cli-docs.sh: that script checks CLI flag coverage,
# while this direction checks compatibility documentation against Cargo.toml.

set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$root"

readme="README.md"
if [ ! -f "$readme" ]; then
    echo "check-compatibility-docs: expected file not found: $readme" >&2
    exit 1
fi

manifest_versions=()
while IFS= read -r version; do
    manifest_versions+=("$version")
done < <(
    grep -R -hE --include='Cargo.toml' \
        '^[[:space:]]*(soroban-sdk|stellar-xdr)[[:space:]]*=' . \
        | grep -oE '"[0-9]+\.[0-9]+\.[0-9]+"' \
        | tr -d '"' \
        | sort -u
)

missing=0
for version in "${manifest_versions[@]}"; do
    if ! grep -qF -- "$version" "$readme"; then
        echo "missing: manifest version $version is not documented in $readme"
        missing=1
    fi
done

compatibility="$(sed -n '/^## ⚙️ Supported Versions & Compatibility/,/^### What "Supported" means here/p' "$readme")"
while IFS= read -r version; do
    if [ -n "$version" ] && ! grep -R -qF --include='Cargo.toml' -- "$version" .; then
        echo "missing: documented version $version is absent from Cargo.toml"
        missing=1
    fi
done < <(printf '%s\n' "$compatibility" | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | sort -u)

xdr_major="$(
    grep -R -hE --include='Cargo.toml' '^[[:space:]]*stellar-xdr[[:space:]]*=' . \
        | grep -oE '"[0-9]+' | head -n1 | tr -d '"'
)"
protocol="$(printf '%s\n' "$compatibility" | grep -oE 'Protocol [0-9]+' | head -n1 | awk '{print $2}')"
if [ -z "$xdr_major" ] || [ -z "$protocol" ] || [ "$protocol" -ne "$xdr_major" ]; then
    echo "mismatch: README protocol $protocol does not match stellar-xdr major $xdr_major"
    missing=1
fi

if [ "$missing" -ne 0 ]; then
    exit 1
fi

echo "compatibility docs: README pins match Cargo.toml manifests"

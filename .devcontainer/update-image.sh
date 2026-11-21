#!/usr/bin/env bash
# Rebuild the shared devcontainer image used by every project that has a copy
# of these .devcontainer files.
#
# Run this after editing Containerfile, or periodically to pull in Arch package
# updates. Projects pick the new image up automatically the next time their
# devcontainer is opened (see podman-gc.sh).
set -euo pipefail

image="localhost/arch-agent-sandbox"
dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

podman build \
    --pull=always \
    --file "${dir}/Containerfile" \
    --tag "${image}:latest" \
    "${dir}"

# The previous generation just lost its tag; remove it unless a container still
# references it (those are retired by podman-gc.sh on the next devcontainer
# open).
podman image prune --force >/dev/null || true

printf 'rebuilt %s:latest\n' "$image"

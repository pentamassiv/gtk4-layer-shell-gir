#!/usr/bin/env bash
# Host-side cleanup for the shared devcontainer image workflow.
#
# Runs from devcontainer.json's `initializeCommand` before the container is
# created/started:
#
#   1. Retires stopped devcontainer containers that predate the current shared
#      image, so each project is recreated from :latest without any manual
#      config change.
#   2. Prunes dangling (untagged) images, i.e. superseded image generations.
#
# Running containers and the workspace bind mount are never touched.
set -uo pipefail

image="${PODMAN_DEV_IMAGE:-localhost/arch-agent-sandbox:latest}"

log() { printf 'podman-gc: %s\n' "$*"; }

if ! command -v podman >/dev/null 2>&1; then
    log "podman not found; skipping"
    exit 0
fi

# --- 1. Retire containers created before the current image was built -------
img_created="$(podman image inspect --format '{{.Created}}' "$image" 2>/dev/null || true)"
img_epoch=""
if [ -n "$img_created" ]; then
    img_epoch="$(date -d "$img_created" +%s 2>/dev/null || true)"
fi

if [ -n "$img_epoch" ]; then
    while IFS= read -r cid; do
        [ -n "$cid" ] || continue

        c_created="$(podman container inspect --format '{{.Created}}' "$cid" 2>/dev/null || true)"
        [ -n "$c_created" ] || continue

        c_epoch="$(date -d "$c_created" +%s 2>/dev/null || true)"
        [ -n "$c_epoch" ] || continue

        if [ "$c_epoch" -lt "$img_epoch" ]; then
            # Without --force, podman refuses to remove running containers, so
            # projects that are concurrently open are left untouched.
            if podman rm "$cid" >/dev/null 2>&1; then
                log "retired stale container $cid"
            fi
        fi
    done < <(podman ps -aq --filter label=devcontainer.local_folder)
fi

# --- 2. Drop superseded image generations ----------------------------------
# Without --all this only removes untagged images, so the tagged shared image
# is always preserved.
if podman image prune --force >/dev/null; then
    log "pruned dangling images"
else
    log "warning: dangling image prune failed" >&2
fi

exit 0

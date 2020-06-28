#!/usr/bin/env bash

SELF="$(dirname "$0")"
ROOT="$(cd "$SELF/../" && pwd)"
IMG_NAME="mlog"

set -exuo pipefail
cd "$ROOT"
docker build -t "$IMG_NAME" .
exec docker run --rm -it -v "$ROOT:/srv/mlog" -w /srv/mlog --cap-add CAP_SYS_ADMIN $IMG_NAME bash

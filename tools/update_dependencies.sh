#!/bin/bash
set -euo pipefail

set -x

cargo raze
buildifier src/main/rust/BUILD src/main/rust/bin/BUILD

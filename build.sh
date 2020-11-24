#!/bin/bash

# Change to the directory that this script is in.
# This allows you to run this script from anywhere and have it still work.
cd $(dirname $0)

# ./build.sh
if [ -z "$RELEASE"  ]; then
  # --------------------------------------------------
  # DEVELOPMENT BUILD
  # --------------------------------------------------

  wasm-pack build -t no-modules --no-typescript -d web --out-name rustcraft.wasm --dev

# RELEASE=1 ./build.sh
else

  # --------------------------------------------------
  # RELEASE BUILD
  # --------------------------------------------------

  # Build the webgl_water_tutorial.wasm file
  wasm-pack build -t no-modules --no-typescript -d web --out-name rustcraft.wasm --release
fi

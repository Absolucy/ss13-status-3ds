#!/usr/bin/env bash
# builds a .cia you can install with FBI, using the bannertool binary at
# ~/.local/bin/bannertool and makerom on PATH.
set -euo pipefail
cd "$(dirname "$0")/.."

APP_TITLE="SS13 Status Viewer"
APP_PRODUCT_CODE="CTR-HB-SS13"
APP_UNIQUE_ID="0xE6B1F"
BANNERTOOL="${BANNERTOOL:-$HOME/.local/bin/bannertool}"

cargo 3ds build --release

mkdir -p target/cia
"$BANNERTOOL" makebanner \
	-i meta/banner.png \
	-a meta/silence.wav \
	-o target/cia/banner.bnr
"$BANNERTOOL" makesmdh \
	-s "$APP_TITLE" \
	-l "Thingymajig to view the status of SS13 servers... on your 3DS!" \
	-p "Lucy" \
	-i logo.png \
	-o target/cia/icon.icn

makerom -f cia \
	-o target/cia/ss13-status-3ds.cia \
	-rsf meta/app.rsf \
	-DAPP_TITLE="$APP_TITLE" \
	-DAPP_PRODUCT_CODE="$APP_PRODUCT_CODE" \
	-DAPP_UNIQUE_ID="$APP_UNIQUE_ID" \
	-DAPP_ROMFS=romfs \
	-elf target/armv6k-nintendo-3ds/release/ss13-status-3ds.elf \
	-icon target/cia/icon.icn \
	-banner target/cia/banner.bnr \
	-exefslogo \
	-target t

echo "built target/cia/ss13-status-3ds.cia"

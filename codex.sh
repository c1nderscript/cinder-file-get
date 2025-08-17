#!/usr/bin/env bash
set -euo pipefail

if [[ ${EUID:-$(id -u)} -eq 0 && "${ALLOW_ROOT:-0}" -ne 1 ]]; then
	echo "Refusing to run as root; use ALLOW_ROOT=1 to override" >&2
	exit 1
fi

DRY_RUN=1
if [[ "${1:-}" == "--confirm" ]]; then
	DRY_RUN=0
	shift
fi

run() {
	if [[ $DRY_RUN -eq 1 ]]; then
		echo "[dry-run] $*"
	else
		"$@"
	fi
}

cmd=${1:-help}
shift || true

case "$cmd" in
bootstrap)
	run cargo build --release
	;;
fast-validate)
	run cargo test
	;;
cache-warm)
	run cargo build
	;;
daemon:status)
	run echo daemon status
	;;
daemon:start)
	run echo starting daemon
	;;
daemon:stop)
	run echo stopping daemon
	;;
codex:repair)
	run cargo fix --allow-dirty
	;;
*)
	echo "Usage: $0 [--confirm] {bootstrap|fast-validate|cache-warm|daemon:{status|start|stop}|codex:repair}"
	;;
esac

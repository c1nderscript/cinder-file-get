#!/usr/bin/env bash
set -euo pipefail

# default to dry-run
mode="dry-run"
if [[ "${1:-}" == "--confirm" ]]; then
	mode="confirm"
	shift
elif [[ "${1:-}" == "--dry-run" ]]; then
	shift
fi

if [[ $EUID -eq 0 && $mode == "confirm" ]]; then
	echo "Refusing to run destructive operations as root without --dry-run" >&2
	exit 1
fi

cmd="${1:-}"
case "$cmd" in
bootstrap)
	echo "[$mode] bootstrap"
	;;
fast-validate)
	echo "[$mode] fast-validate"
	;;
cache-warm)
	echo "[$mode] cache-warm"
	;;
daemon:status)
	echo "[$mode] daemon status"
	;;
daemon:start)
	if [[ $mode == "dry-run" ]]; then
		echo "[$mode] daemon start"
	else
		echo "daemon start"
	fi
	;;
daemon:stop)
	if [[ $mode == "dry-run" ]]; then
		echo "[$mode] daemon stop"
	else
		echo "daemon stop"
	fi
	;;
codex:repair)
	echo "[$mode] codex repair"
	;;
*)
	echo "Usage: $0 [--dry-run|--confirm] {bootstrap|fast-validate|cache-warm|daemon:status|daemon:start|daemon:stop|codex:repair}" >&2
	exit 1
	;;
esac

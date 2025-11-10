#!/usr/bin/env bash
set -euo pipefail

usage() {
    cat <<USAGE >&2
Usage:
  $(basename "$0") --bin <name> [-- <arg>...]
  $(basename "$0") --example <name> [-- <arg>...]
  $(basename "$0") --path <binary-path> [-- <arg>...]

Always builds the requested target with "cargo build --release" for the
m68k-atari-mintelf triple, then boots it inside Hatari using
scripts/hatari-runner.sh.
USAGE
    exit 1
}

[[ $# -ge 2 ]] || usage

artifact_kind=$1
shift

case "$artifact_kind" in
    --bin|--example)
        [[ $# -ge 1 ]] || usage
        name=$1
        shift
        ;;
    --path)
        [[ $# -ge 1 ]] || usage
        binary_path=$1
        shift
        ;;
    *)
        usage
        ;;
esac

args=()
if [[ $# -gt 0 ]]; then
    if [[ $1 == "--" ]]; then
        shift
    fi
    args=("$@")
fi

cargo_target=m68k-atari-mintelf
profile=release
out_dir="target/${cargo_target}/${profile}"

if [[ "$artifact_kind" != "--path" ]]; then
    if [[ "$artifact_kind" == "--bin" ]]; then
        cargo build --release --bin "${name}"
        binary_path="${out_dir}/${name}"
    elif [[ "$artifact_kind" == "--example" ]]; then
        cargo build --release --example "${name}"
        binary_path="${out_dir}/examples/${name}"
    fi
fi

if [[ ! -x "${binary_path}" ]]; then
    echo "binary \"${binary_path}\" not found" >&2
    exit 1
fi

scripts/hatari-runner.sh "${binary_path}" "${args[@]}"

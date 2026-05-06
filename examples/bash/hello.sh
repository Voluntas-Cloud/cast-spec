#!/usr/bin/env bash
# Shell branch of cast's examples corpus. Pointed at by the
# `bash_examples` concept via cast::io::continues_in!.

greet() {
    local name="${1:-World}"
    echo "Hello, ${name}!"
}

greet "$@"

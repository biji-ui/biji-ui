#!/usr/bin/env bash
leptosfmt --stdin | \
    rustywind --stdin | \
    rustywind --stdin --custom-regex "\b\w*class=(?:cn!\([^\"]*\")?([_a-zA-Z0-9\s\-:\[\]/\.]+)\"" | \
    rustfmt --edition 2024

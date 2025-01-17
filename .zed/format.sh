#!/usr/bin/env bash
leptosfmt --stdin | \
    rustywind --stdin --custom-regex "\b(?:class(?:Name)*\s*=\s*[\"'])([_a-zA-Z0-9\.\s\-:\[\]/]+)[\"']" --output-css-file "$(pwd)/style/main.css" | \
    rustywind --stdin --custom-regex "\b(?:const\s\b.*_STYLE:\s&str\s*=\s*[\"'])([_a-zA-Z0-9\.\s\-:\[\]/]+)[\"']" --output-css-file "$(pwd)/style/main.css" | \
    rustfmt --edition 2021

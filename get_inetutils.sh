#!/bin/bash

if [ ! -f "inetutils-2.0.tar.xz" ]; then
    echo "Downloading inetutils-2.0.tar.xz..."
    wget https://ftp.gnu.org/gnu/inetutils/inetutils-2.0.tar.xz
    echo "inetutils-2.0.tar.xz downloaded."
fi

if [ ! -d "inetutils-2.0" ]; then
    echo "Extracting..."
    tar xf inetutils-2.0.tar.xz
    echo "Extracted."
fi

if [ ! -f "inetutils-2.0/ping/Makefile" ]; then
    echo "Fixing wrong argp_parse error print"
    sed -i "s/Try '%s --help' or '%s --usage' for more information\\.\\\\n\"),/Try \`%s --help' or \`%s --usage' for more information\\.\\\\n\"),/" inetutils-2.0/lib/argp-help.c;
    pushd inetutils-2.0;
    echo "Compiling..."
    ./configure;
    make;
    echo "Compiled."
    popd;
fi


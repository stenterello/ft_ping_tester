#!/bin/bash

if [ ! -f "inetutils-2.0.tar.xz" ]; then
    wget https://ftp.gnu.org/gnu/inetutils/inetutils-2.0.tar.xz
fi

if [ ! -d "inetutils-2.0" ]; then
    tar xf inetutils-2.0.tar.xz
fi

if [ ! -f "inetutils-2.0/ping/Makefile" ]; then
    sed -i "s/Try '%s --help' or '%s --usage' for more information\\.\\\\n\"),/Try \`%s --help' or \`%s --usage' for more information\\.\\\\n\"),/" inetutils-2.0/lib/argp-help.c;
    pushd inetutils-2.0;
    ./configure;
    make;
    popd;
fi

make -C ../ft_ping

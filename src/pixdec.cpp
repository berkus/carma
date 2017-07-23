//
// Part of Roadkill Project. Check http://<urlhere> for latest version.
//
// Copyright 2010, Stanislav Karchebnyy <berkus@exquance.com>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#include "blocks.h"
#include <cstdio>

using namespace std;
using raii_wrapper::file;

int main(int argc, char **argv)
{
    if (argc < 2)
    {
        printf("\n\n ERROR!!! File name required.\n\n");
        return 1;
    }

    file f(argv[1], ios::in|ios::binary);

    if (!resource_file_t::read_file_header(f))
    {
        printf("\n\n ERROR!!! File header truncated.\n\n");     //exit if file header short
        return 1;
    }

    pixelmap_t pixelmap;

    while (pixelmap.read(f))
        pixelmap.dump();

    return 0;
}

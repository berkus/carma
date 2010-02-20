#include "blocks.h"
#include <cstdio>

using namespace std;
using namespace raii_wrapper;

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

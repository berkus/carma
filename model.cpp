#include "blocks.h"
#include <cstdio>

using namespace std;
using raii_wrapper::file;

int main(int argc, char **argv)
{
    if (argc < 2)       //Was a file name specified?
    {
        printf("\n\n ERROR!!! File name required\n\n");
        return 1;
    }

    file f(argv[1], ios::in|ios::binary);

    if (!resource_file_t::read_file_header(f))
    {
        printf("\n\n ERROR!!! File header truncated.\n\n");     //exit if file header short
        return 1;
    }

    mesh_t mesh;

    while (mesh.read(f))
        mesh.dump();

    return 0;
}

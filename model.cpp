#include "blocks.h"
#include <cstdio>

using namespace std;
using raii_wrapper::file;

unsigned char file_header[16];   //      The file header

int main(int argc, char **argv)
{
    size_t count;

    if (argc < 2)       //Was a file name specified?
    {
        printf("\n\n ERROR!!!   ");
        puts("File name required\n");
        return(1);    //if not, exit
    }

    file f(argv[1], ios::in|ios::binary);

    count = f.read(file_header, 16);        //Read 16 byte file header

    if(count < 16)
    {
        puts("\n\n ERROR!!!  File header truncated.\n");     //exit if file header short
        return(1);
    }

    printf("File header Data: ");      //Print file header to the screen
    for(int loop=0;loop<16;loop++)
    {
        printf("%02hX ",(file_header[loop]));
    }
    puts("\nReading Chunks:");

    mesh_t mesh;

    while (mesh.read(f))
        mesh.dump();

    return(0);                         //Exit program
}

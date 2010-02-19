#include "blocks.h"
#include <cstdio>
#include <cstring>

using namespace std;
using namespace raii_wrapper;

unsigned char file_header[16];

int main(int argc, char **argv)
{
    size_t count;

    if (argc < 2)
    {
        printf("\n\n ERROR!!! File name required.\n\n");
        return 1;
    }

    file f(argv[1], ios::in|ios::binary);
    filebinio fio(f);

    count = f.read(file_header, 16);        //Read 16 byte file header

    if(count < 16)
    {
        printf("\n\n ERROR!!! File header truncated.\n");
        return 1;
    }

    printf("File header Data: ");      //Print file header to the screen
    for(int loop=0;loop<16;loop++)
    {
        printf("%02hX ",(file_header[loop]));
    }
    puts("\nReading Chunks:");

    chunk_header_t ch;

    ch.read(f);
    uint8_t marker;
    uint16_t w, h, what, half_w, half_h;
    string str;
    fio.read8(marker);
    fio.read16be(w);
    fio.read16be(h);
    fio.read16be(what);
    fio.read16be(half_w);
    fio.read16be(half_h);
    chunk_header_t::read_c_string(f, str);

    ch.read(f);
    uint32_t chunk_size, payload_size;
    fio.read32be(chunk_size);
    fio.read32be(payload_size);

    char* buf = new char [payload_size];
    f.read(buf, payload_size);

    const char* arr = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.,_!@#$%^&*()_+";
    size_t sz = strlen(arr);

    for (int y = 0; y < 64; y++)
    {
        for (int x = 0; x < 64; x++)
        {
            printf("%c", arr[buf[y*64+x] % sz]);
        }
        printf("\n");
    }
//     pixmap_t pixelmap;

//     while (pixelmap.read(f))
//         pixelmap.dump();

    return 0;
}

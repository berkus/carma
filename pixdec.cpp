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
    uint16_t w, h, what, use_w, use_h;
    string str;
    fio.read8(marker);
    fio.read16be(w);
    fio.read16be(use_w);
    fio.read16be(h);
    fio.read16be(use_h);
    fio.read16be(what);
    chunk_header_t::read_c_string(f, str);

    printf("Pixmap header chunk [type %02x, size %d]: %s (%d, %d[%d] x %d[%d], %d)\n", ch.type, ch.size, str.c_str(), marker, w, use_w, h, use_h, what);

    ch.read(f);
    uint32_t payload_size, whatnot;
    fio.read32be(payload_size);
    fio.read32be(whatnot);

    printf("Pixmap data chunk [type %02x, size %d]: payload %d, whatnot %d\n", ch.type, ch.size, payload_size, whatnot);

    char* buf = new char [payload_size];
    f.read(buf, payload_size);

    const char* arr = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.,_!@#$%^&*()_+";
    size_t sz = strlen(arr);

    for (int y = 0; y < h; y++)
    {
        for (int x = 0; x < use_w; x++)
        {
            printf("%c", arr[buf[y*w+x] % sz]);
        }
        printf("\n");
    }
//     pixmap_t pixelmap;

//     while (pixelmap.read(f))
//         pixelmap.dump();

    return 0;
}

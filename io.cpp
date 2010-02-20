#include "raiifile.h"
#include "blocks.h"
#include <stdio.h>

#define MATERIAL_LIST 0x16
#define VERTEX_LIST   0x17
#define UVMAP_LIST    0x18
#define FACE_LIST     0x35
#define FILE_NAME     0x36
#define FACE_MAT_LIST 0x1a

#define PIXELMAP_HEAD 0x03
#define PIXELMAP_DATA 0x21

using namespace std;
using namespace raii_wrapper;

// TODO: make non-static and add dump()?
bool resource_file_t::read_file_header(file& f)
{
    size_t count;
    unsigned char file_header[16];   //      The file header

    count = f.read(file_header, 16);        //Read 16 byte file header

    if(count < 16)
        return false;
/*    {
        puts("\n\n ERROR!!!  File header truncated.\n");     //exit if file header short
        return false;
    }

    printf("File header Data: ");      //Print file header to the screen
    for(int loop=0;loop<16;loop++)
    {
        printf("%02hX ",(file_header[loop]));
    }*/

    return true;
}

bool resource_file_t::read_c_string(file& f, std::string& str)
{
    filebinio fio(f);
    str = "";
    int8_t datum = 1;
    
    while (datum)
    {
        CHECK_READ(fio.read8(datum));
        str.push_back(datum);
    }
    
    return true;
}

bool chunk_header_t::read(file& f)
{
    filebinio fio(f);
    CHECK_READ(fio.read32be(type));
    CHECK_READ(fio.read32be(size));
    return true;
}

bool chunk_t::read(file& f)
{
    filebinio fio(f);
    CHECK_READ(fio.read32be(type));
    CHECK_READ(fio.read32be(size));

    if (type == FILE_NAME) // no entries in this chunk header
    {
        int16_t dummy;
        CHECK_READ(fio.read16be(dummy));
        entries = dummy;
//         size -= 2;
    }
    else
    {
        CHECK_READ(fio.read32be(entries));
//         size -= 4;
    }

    if (type == FACE_MAT_LIST) size += 8;

    return true;
}

#define fix2float(x) *reinterpret_cast<float*>(&x)

bool vertex_t::read(file& f)
{
    filebinio fio(f);
    int32_t datum;
    CHECK_READ(fio.read32be(datum));
    x = fix2float(datum);
    CHECK_READ(fio.read32be(datum));
    y = fix2float(datum);
    CHECK_READ(fio.read32be(datum));
    z = fix2float(datum);
    return true;
}

bool uvcoord_t::read(file& f)
{
    filebinio fio(f);
    int32_t datum;
    CHECK_READ(fio.read32be(datum));
    u = fix2float(datum);
    CHECK_READ(fio.read32be(datum));
    v = fix2float(datum);
    return true;
}

bool face_t::read(file& f)
{
    filebinio fio(f);
    CHECK_READ(fio.read16be(v1));
    CHECK_READ(fio.read16be(v2));
    CHECK_READ(fio.read16be(v3));
    CHECK_READ(fio.read16be(flags));
    CHECK_READ(fio.read8(unknown));
    return true;
}

bool mesh_t::read(file& f)
{
    vertices.clear();
    uvcoords.clear();
    faces.clear();
    materials.clear();

    filebinio fio(f);
    chunk_t header;
    uint32_t dummy;

    printf("Reading filename entry...\n");
    CHECK_READ(header.read(f));

    if (header.type != FILE_NAME)
        return false;

    char* s = new char [header.size - 2];
    if (f.read(s, header.size - 2) < header.size - 2)
    {
        delete s;
        return false;
    }
    name = string(s);
    delete s;

    printf("Reading vertex list...\n");
    CHECK_READ(header.read(f));

    if (header.type != VERTEX_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        vertex_t v;
        CHECK_READ(v.read(f));
        vertices.push_back(v);
    }

    printf("Reading uvmap list...\n");
    CHECK_READ(header.read(f));

    if (header.type != UVMAP_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        uvcoord_t v;
        CHECK_READ(v.read(f));
        uvcoords.push_back(v);
    }

    printf("Reading face list...\n");
    CHECK_READ(header.read(f));

    if (header.type != FACE_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        face_t v;
        CHECK_READ(v.read(f));
        faces.push_back(v);
    }

    printf("Reading material list...\n");
    CHECK_READ(header.read(f));

    if (header.type != MATERIAL_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        string str;
        CHECK_READ(resource_file_t::read_c_string(f, str));
        materials.push_back(str);
    }

    printf("Reading face material list...\n");
    CHECK_READ(header.read(f));

    if (header.type != FACE_MAT_LIST)
        return false;

    CHECK_READ(fio.read32be(dummy));
    for (size_t s = 0; s < header.entries; s++)
    {
        CHECK_READ(fio.read16be(faces[s].material_id));
    }
    CHECK_READ(fio.read32be(dummy));
    CHECK_READ(fio.read32be(dummy));

    return true;
}

bool pixelmap_t::read(raii_wrapper::file& f)
{
    filebinio fio(f);
    chunk_header_t ch;
    uint32_t dummy;

    CHECK_READ(ch.read(f));
    if (ch.type != PIXELMAP_HEAD)
        return false;
    fio.read8(what1);
    fio.read16be(w);
    fio.read16be(use_w);
    fio.read16be(h);
    fio.read16be(use_h);
    fio.read16be(what2);
    CHECK_READ(resource_file_t::read_c_string(f, name));

    CHECK_READ(ch.read(f));
    if (ch.type != PIXELMAP_DATA)
        return false;

    fio.read32be(payload_size);
    fio.read32be(what3);

    data = new char [payload_size];
    if (!data)
        return false;
    if (f.read(data, payload_size) < payload_size)
        return false;
    CHECK_READ(fio.read32be(dummy));
    CHECK_READ(fio.read32be(dummy));

    return true;
}

#include "raiifile.h"
#include "blocks.h"

#define MATERIAL_LIST 0x16
#define VERTEX_LIST   0x17
#define UVMAP_LIST    0x18
#define FACE_LIST     0x35
#define FILE_NAME     0x36
#define FACE_MAT_LIST 0x1a

using namespace std;
using namespace raii_wrapper;

bool chunk_t::read(file& f)
{
    filebinio fio(f);
    fio.read32be(type);
    fio.read32be(size);

    if (type == FILE_NAME) // no entries in this chunk header
    {
        int16_t dummy;
        fio.read16be(dummy);
        entries = dummy;
//         size -= 2;
    }
    else
    {
        fio.read32be(entries);
//         size -= 4;
    }

    if (type == FACE_MAT_LIST) size += 8;

    return true;
}

static inline float conv_fixed_16_16(int32_t fx)
{
    double fp = fx;
    fp = fp / ((double)(1<<16)); // multiplication by a constant
    return fp;
}

bool vertex_t::read(file& f)
{
    filebinio fio(f);
    int32_t datum;
    fio.read32be(datum);
    x = conv_fixed_16_16(datum);
    fio.read32be(datum);
    y = conv_fixed_16_16(datum);
    fio.read32be(datum);
    z = conv_fixed_16_16(datum);
    return true;
}

bool uvcoord_t::read(file& f)
{
    filebinio fio(f);
    int32_t datum;
    fio.read32be(datum);
    u = conv_fixed_16_16(datum);
    fio.read32be(datum);
    v = conv_fixed_16_16(datum);
    return true;
}

bool face_t::read(file& f)
{
    filebinio fio(f);
    fio.read16be(v1);
    fio.read16be(v2);
    fio.read16be(v3);
    fio.read16be(flags);
    fio.read8(unknown);
    return true;
}

bool mesh_t::read(file& f)
{
    vertices.clear();
    uvcoords.clear();
    faces.clear();
    materials.clear();

    chunk_t header;
    uint32_t dummy;
    filebinio fio(f);

    if (!header.read(f))
        return false;

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

    if (!header.read(f))
        return false;

    if (header.type != VERTEX_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        vertex_t v;
        if (!v.read(f))
            return false;
        vertices.push_back(v);
    }

    if (!header.read(f))
        return false;

    if (header.type != UVMAP_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        uvcoord_t v;
        if (!v.read(f))
            return false;
        uvcoords.push_back(v);
    }

    if (!header.read(f))
        return false;

    if (header.type != FACE_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        face_t v;
        if (!v.read(f))
            return false;
        faces.push_back(v);
    }

    if (!header.read(f))
        return false;

    if (header.type != MATERIAL_LIST)
        return false;

    for (size_t s = 0; s < header.entries; s++)
    {
        string str;
        int8_t datum = 1;

        while (datum)
        {
            fio.read8(datum);//FIXME: return bool from readN()
            str.push_back(datum);
        }

        materials.push_back(str);
    }

    if (!header.read(f))
        return false;

    if (header.type != FACE_MAT_LIST)
        return false;

    fio.read32be(dummy);
    for (size_t s = 0; s < header.entries; s++)
    {
        fio.read16be(faces[s].material_id);
    }
    fio.read32be(dummy);
    fio.read32be(dummy);

    return true;
}

#pragma once

#include "raiifile.h"
#include <vector>

class chunk_t
{
public:
    uint32_t type;     //chunk type
    uint32_t size;     //size of chunk -4
    uint32_t entries;  //number of entires

    bool read(raii_wrapper::file& f);
};

class vertex_t
{
public:
    float x, y, z;

    bool read(raii_wrapper::file& f);
};

class uvcoord_t
{
public:
    float u, v;

    bool read(raii_wrapper::file& f);
};

class face_t
{
public:
    int16_t v1, v2, v3; // vertex indices (works with glDrawElements() e.g.)
    int16_t flags; // looks like flags, always only one bit set -- not always, see CITYA81.DAT!!
    int8_t unknown; // something, no idea yet, might be related to flags
    int16_t material_id;

    bool read(raii_wrapper::file& f);
};

class mesh_t
{
public:
    std::string name;
    std::vector<vertex_t> vertices;
    std::vector<uvcoord_t> uvcoords;
    std::vector<face_t> faces;
    std::vector<std::string> materials;

    bool read(raii_wrapper::file& f);
    void dump();
};

#pragma once

#include "raiifile.h"
#include <vector>

#define CHECK_READ(v)  if(!(v)) return false

class resource_file_t
{
public:
    /* Helper to read resource file header. */
    static bool read_file_header(raii_wrapper::file& f);

    /* Helper to read C strings */
    static bool read_c_string(raii_wrapper::file& f, std::string& str);
};

// File structures.
class chunk_header_t
{
public:
    uint32_t type;     //chunk type
    uint32_t size;     //size of chunk -4

    bool read(raii_wrapper::file& f);
};

class chunk_t
{
public:
    uint32_t type;     //chunk type
    uint32_t size;     //size of chunk -4
    uint32_t entries;  //number of entires  -- only in DAT, not part of chunk header actually (different for other types)

    bool read(raii_wrapper::file& f);
};

// Meshes.
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

// Materials.

// MAT file is an index of: material internal name, PIX file name and TAB file name.

// Pixmap consists of two chunks: name and data
class pixelmap_t
{
public:
    std::string name;
    uint16_t w, h, use_w, use_h; /* Actual texture w & h and how much of that is used for useful data */
    uint8_t what1;
    uint16_t what2;
    uint32_t payload_size, what3;
    char* data;

    bool read(raii_wrapper::file& f);
    void dump();
};

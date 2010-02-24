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
#include <cstring>

void mesh_t::dump()
{
    printf("Name: %s\n", name.c_str());
    // Print vertices.
    for (size_t i = 0; i < vertices.size(); i++)
    {
        printf("Vertex{%f,%f,%f}\n", vertices[i].x, vertices[i].y, vertices[i].z);
    }
    for (size_t i = 0; i < faces.size(); i++)
    {
        printf("Face{%d,%d,%d} Mat: %s\n", faces[i].v1, faces[i].v2, faces[i].v3, faces[i].material_id == 0 ? "<DEFAULT?>" : material_names[faces[i].material_id-1].c_str());
    }
}

void material_t::dump()
{
    printf("Material %s (%s/%s) values:\n", name.c_str(), pixelmap_name.c_str(), rendertab_name.c_str());
    for (int i = 0; i < 12; i++)
        printf("%f\n", params[i]);
}

void pixelmap_t::dump()
{
    printf("Pixmap header: %s (what1 %d, %d[%d] x %d[%d], what2 %d)\n", name.c_str(), what1, w, use_w, h, use_h, what2);
    printf("Pixmap data: payload %d units x %d bytes\n", units, unit_bytes);

    const char* arr = " .,_+*^!$@#%&abcdefghijklmnopqrstuvwxyz0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    size_t sz = strlen(arr);

    for (int y = 0; y < h; y++)
    {
        for (int x = 0; x < use_w; x++)
        {
            printf("%c", arr[data[y*w+x] % sz]);
        }
        printf("\n");
    }
}

void actor_t::dump()
{
    printf("Actor: %s, visible %d, what2 %d, mesh %s, material %s\n", name.c_str(), what1, what2, mesh_name.c_str(), material_name.c_str());
    for (size_t i = 0; i < 12; ++i)
        printf("%f  ", values[i]);
    printf("\n");
}

void model_t::dump()
{
    for(std::map<std::string, actor_t*>::iterator it = parts.begin(); it != parts.end(); ++it)
        (*it).second->dump();
    printf("\n");
}

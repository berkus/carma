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
    printf("Pixmap data: payload %d bytes, what3 %d\n", payload_size, what3);

    const char* arr = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.,_!@#$%^&*()_+";
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

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
        printf("Face{%d,%d,%d} Mat: %s\n", faces[i].v1, faces[i].v2, faces[i].v3, faces[i].material_id == 0 ? "<DEFAULT?>" : materials[faces[i].material_id-1].c_str());
    }
}

void pixelmap_t::dump()
{
    printf("Pixmap header: %s (what1 %d, %d[%d] x %d[%d], what2 %d)\n", name.c_str(), what1, w, use_w, h, use_h, what2);
    printf("Pixmap data: payload %d bytes, what3 %d\n", payload_size, what3);

/*    const char* arr = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz.,_!@#$%^&*()_+";
    size_t sz = strlen(arr);

    for (int y = 0; y < h; y++)
    {
        for (int x = 0; x < use_w; x++)
        {
            printf("%c", arr[buf[y*w+x] % sz]);
        }
        printf("\n");
    }*/
}

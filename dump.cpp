#include "blocks.h"
#include <stdio.h>

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

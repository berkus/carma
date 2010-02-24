//
// Part of Roadkill Project. Check http://<urlhere> for latest version.
//
// Copyright 2010, Stanislav Karchebnyy <berkus@exquance.com>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#include "blocks.h"
#include <map>

class texture_t
{
public:
    GLuint bound_id;
    pixelmap_t pixelmap;

    texture_t() : bound_id(0), pixelmap() {}
};

class texture_renderer_t
{
public:
    std::map<std::string, texture_t*> cache;

    bool read(raii_wrapper::file& f);
    bool set_texture(std::string name);
    bool draw_texture(std::string name);
    void dump_cache();

    /* Set palette for converting GL_COLOR_INDEX pixmaps to textures. */
//     GL_COLOR_INDEX
//     Each element is a single value, a color index. The GL converts it to fixed point (with an unspecified number of zero bits to the right of the binary point), shifted left or right depending on the value and sign of GL_INDEX_SHIFT, and added to GL_INDEX_OFFSET (see glPixelTransfer). The resulting index is converted to a set of color components using the GL_PIXEL_MAP_I_TO_R, GL_PIXEL_MAP_I_TO_G, GL_PIXEL_MAP_I_TO_B, and GL_PIXEL_MAP_I_TO_A tables, and clamped to the range [0,1].
    bool set_palette(pixelmap_t palette);
};

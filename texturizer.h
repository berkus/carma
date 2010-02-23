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
};

#include <cstring>
#include "blocks.h"

/* This is inefficient but safe. Perform deep copy. */

pixelmap_t::pixelmap_t(const pixelmap_t& other)
    : data(0)
{
    (*this).operator =(other);
}

pixelmap_t& pixelmap_t::operator =(const pixelmap_t& other)
{
    if (this != &other)
    {
        name = other.name;
        w = other.w;
        h = other.h;
        use_w = other.use_w;
        use_h = other.use_h;
        what1 = other.what1;
        what2 = other.what2;
        units = other.units;
        unit_bytes = other.unit_bytes;
        delete data; data = new uint8_t [units * unit_bytes];
        memcpy(data, other.data, units * unit_bytes);
    }
    return *this;
}

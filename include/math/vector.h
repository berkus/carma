//
// Part of Roadkill Project. Check http://<urlhere> for latest version.
//
// Copyright 1998 - 2010, Stanislav Karchebnyy <berkus@exquance.com>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#pragma once

#include <cmath>
#include <cstdlib>
#include <algorithm>
#include "raiifile.h"

template <typename type_t>
class vector_t
{
public:
    type_t x, y, z;

    vector_t() {}
    vector_t(type_t v) { x = y = z = v; }
    vector_t(const vector_t& v) { x = v.x;y = v.y;z = v.z; }
    vector_t(type_t vx,type_t vy,type_t vz) { x = vx;y = vy;z = vz; }
    vector_t& operator = (const vector_t& v) { x = v.x;y = v.y; z = v.z; return *this; }
    vector_t& operator = (type_t v) { x=y=z=v; return *this; }
    vector_t operator - () const;
    vector_t& operator += (const vector_t&);
    vector_t& operator -= (const vector_t&);
    vector_t& operator *= (const vector_t&);
    vector_t& operator *= (type_t);
    vector_t& operator /= (type_t);

    //!  Dot product -- gives scalar angle between vectors
    friend type_t operator ^(const vector_t<type_t>& u,const vector_t<type_t>& v)
    {
        return u.x * v.x + u.y * v.y + u.z * v.z;
    }
    //!  Cross product -- gives a normal vector to two given vectors.
    friend vector_t<type_t> operator &(const vector_t<type_t>& u, const vector_t<type_t>& v)
    {
        return vector_t<type_t>(u.y * v.z - u.z * v.y, u.z * v.x - u.x * v.z, u.x * v.y - u.y * v.x);
    }
    // length
    inline type_t operator !() { return length(); }
    inline type_t length()  { return (type_t)sqrt(x * x + y * y + z * z); }
    // component
    inline type_t& operator [](int n) { return(*(&x+n)); };
    // comparison
    int operator < (type_t v) { return x < v && y < v && z < v; };
    int operator > (type_t v) { return x > v && y > v && z > v; };
    //        int operator < (const vector_t&);

    static vector_t<type_t> random();

    bool read(raii_wrapper::file& f);
    void dump();
};

/*inline int vector_t::operator < (const vector_t& v)
{
    return x < v.x &&
           y < v.y &&
           z < v.z;
}*/

template <typename type_t>
inline vector_t<type_t> vector_t<type_t>::operator -() const
{
    return vector_t<type_t>(-x, -y, -z);
}

template <typename type_t>
inline vector_t<type_t> operator +(const vector_t<type_t>& u, const vector_t<type_t>& v)
{
    return vector_t<type_t>(u.x + v.x, u.y + v.y, u.z + v.z);
}

template <typename type_t>
inline vector_t<type_t> operator -(const vector_t<type_t>& u, const vector_t<type_t>& v)
{
    return vector_t<type_t>(u.x - v.x, u.y - v.y, u.z - v.z);
}

template <typename type_t>
inline vector_t<type_t> operator *(const vector_t<type_t>& u, const vector_t<type_t>& v)
{
    return vector_t<type_t>(u.x * v.x, u.y * v.y, u.z * v.z);
}

template <typename type_t>
inline vector_t<type_t> operator *(const vector_t<type_t>& u, type_t f)
{
    return vector_t<type_t>(u.x * f, u.y * f, u.z * f);
}

template <typename type_t>
inline vector_t<type_t> operator *(type_t f, const vector_t<type_t>& v)
{
    return vector_t<type_t>(v.x * f, v.y * f, v.z * f);
}

template <typename type_t>
inline vector_t<type_t> operator /(const vector_t<type_t>& u, const vector_t<type_t>& v)
{
    return vector_t<type_t>(u.x / v.x, u.y / v.y, u.z / v.z);
}

template <typename type_t>
inline vector_t<type_t> operator /(const vector_t<type_t>& v, type_t f)
{
    return vector_t<type_t>(v.x / f, v.y / f, v.z / f);
}

template <typename type_t>
inline vector_t<type_t>& vector_t<type_t>::operator +=(const vector_t<type_t>& v)
{
    x += v.x;
    y += v.y;
    z += v.z;
    return *this;
}

template <typename type_t>
inline vector_t<type_t>& vector_t<type_t>::operator -=(const vector_t<type_t>& v)
{
    x -= v.x;
    y -= v.y;
    z -= v.z;
    return *this;
}

template <typename type_t>
inline vector_t<type_t>& vector_t<type_t>::operator *=(type_t v)
{
    x *= v;
    y *= v;
    z *= v;
    return *this;
}

template <typename type_t>
inline vector_t<type_t>& vector_t<type_t>::operator *=(const vector_t<type_t>& v)
{
    x *= v.x;
    y *= v.y;
    z *= v.z;
    return *this;
}

template <typename type_t>
inline vector_t<type_t>& vector_t<type_t>::operator /=(type_t v)
{
    x /= v;
    y /= v;
    z /= v;
    return *this;
}

//
//  Funcs
//
template <typename type_t>
inline vector_t<type_t> normalize(vector_t<type_t> v)
{
    return v / !v;
}

//
//  Halfway between two vector_ts
//
template <typename type_t>
inline vector_t<type_t> halfway(vector_t<type_t> u, vector_t<type_t> v)
{
    return (u + v) * 0.5;
}

//
//  Random vector_t
//
template <typename type_t>
inline vector_t<type_t> vector_t<type_t>::random()
{
    vector_t v(rand() - 0.5 * RAND_MAX, rand() - 0.5 * RAND_MAX, rand() - 0.5 * RAND_MAX);
    return normalize(v);
}

template <typename type_t>
inline vector_t<type_t> clamp(vector_t<type_t> v)
{
    v.x = std::max(0.0, std::min(v.x, 1.0));
    v.y = std::max(0.0, std::min(v.x, 1.0));
    v.z = std::max(0.0, std::min(v.x, 1.0));
    return v;
}

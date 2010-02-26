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
#include <algorithm>
#include "math/vector.h"

template <typename type_t>
class matrix_t
{
public:
    type_t x[4][4]; // [cols][rows]

    matrix_t(type_t = 1.0); // Identity matrix by default

    matrix_t& operator +=(const matrix_t&);
    matrix_t& operator -=(const matrix_t&);
    matrix_t& operator *=(const matrix_t&);
    matrix_t& operator *=(type_t);
    matrix_t& operator /=(type_t);

    void identity();
    void invert();
    void transpose();

    void dump();

    static matrix_t<type_t> translate(const vector_t<type_t>& loc);
    static matrix_t<type_t> inv_translate(const vector_t<type_t>& loc);

    static matrix_t<type_t> scale(const vector_t<type_t>& vec);
    static matrix_t<type_t> inv_scale(const vector_t<type_t>& vec);

    // Counter-clockwise rotation around axes
    static matrix_t<type_t> rotate_x(type_t radians);
    static matrix_t<type_t> rotate_y(type_t radians);
    static matrix_t<type_t> rotate_z(type_t radians);
    static matrix_t<type_t> rotation(const vector_t<type_t>& axis, type_t radians);

    static matrix_t<type_t> mirror_x();
    static matrix_t<type_t> mirror_y();
    static matrix_t<type_t> mirror_z();
};


template <typename type_t>
inline matrix_t<type_t>::matrix_t(type_t v)
{
    for(int col = 0; col < 4; ++col)
        for(int row = 0; row < 4; ++row)
            x[col][row] = (row == col) ? v : 0.0;

    x[3][3] = 1.0;
}

template <typename type_t>
inline void matrix_t<type_t>::identity()
{
    for(int col = 0; col < 4; ++col)
        for(int row = 0; row < 4; ++row)
            x[col][row] = (row == col) ? 1.0 : 0.0;
}

template <typename type_t>
void matrix_t<type_t>::invert()
{
    matrix_t out(1);

    for(int diag = 0; diag < 4; ++diag)
    {
        type_t d = x[diag][diag];

        if(d != 1.0)
        {
            for(int row = 0; row < 4; ++row)
            {
                out.x[diag][row] /= d;
                x[diag][row] /= d;
            }
        }
        for(int col = 0; col < 4; ++col)
        {
            if(col != diag)
            {
                if(x[col][diag] != 0.0)
                {
                    type_t mulby = x[col][diag];

                    for(int row = 0; row < 4; ++row)
                    {
                        x[col][row]     -= mulby * x[diag][row];
                        out.x[col][row] -= mulby * out.x[diag][row];
                    }
                }
            }
        }
    }
    *this = out;
}

template <typename type_t>
void matrix_t<type_t>::transpose()
{
    for(int col = 0; col < 4; ++col)
        for(int row = 0; row < 4; ++row)
            if(row != col)
                std::swap(x[col][row], x[row][col]);
}

template <typename type_t>
matrix_t<type_t>& matrix_t<type_t>::operator +=(const matrix_t<type_t>& a)
{
    for(int col = 0; col < 4; ++col)
        for(int row = 0; row < 4; ++row)
            x[col][row] += a.x[col][row];
    return *this;
}

template <typename type_t>
matrix_t<type_t>& matrix_t<type_t>::operator -=(const matrix_t<type_t>& a)
{
    for(int col = 0; col < 4; ++col)
        for(int row = 0; row < 4; ++row)
            x[col][row] -= a.x[col][row];
    return *this;
}

// Multiply matrix by a scalar v
template <typename type_t>
matrix_t<type_t>& matrix_t<type_t>::operator *=(type_t v)
{
    for(int col = 0; col < 4; ++col)
        for(int row = 0; row < 4; ++row)
            x[col][row] *= v;
    return *this;
}

// Multiply matrices
template <typename type_t>
matrix_t<type_t>& matrix_t<type_t>::operator *=(const matrix_t<type_t>& a)
{
    matrix_t res = *this;

    for(int col = 0; col < 4; ++col)
        for(int row = 0; row < 4; ++row)
        {
            type_t sum = 0.0;

            for(int k = 0; k < 4; k++)
                sum += res.x[col][k] * a.x[k][row];

            x[col][row] = sum;
        }

    return *this;
}

// Friend operators.

// Sum matrices
template <typename type_t>
matrix_t<type_t> operator +(const matrix_t<type_t>& a, const matrix_t<type_t>& b)
{
    matrix_t<type_t> res = a;
    res += b;
    return res;
}

template <typename type_t>
matrix_t<type_t> operator -(const matrix_t<type_t>& a, const matrix_t<type_t>& b)
{
    matrix_t<type_t> res = a;
    res -= b;
    return res;
}

template <typename type_t>
matrix_t<type_t> operator *(const matrix_t<type_t>& a, const matrix_t<type_t>& b)
{
    matrix_t<type_t> res = a;
    res *= b;
    return res;
}

template <typename type_t>
matrix_t<type_t> operator *(const matrix_t<type_t>& a, type_t v)
{
    matrix_t<type_t> res = a;
    res *= v;
    return res;
}

// Multiply matrix by a vector
template <typename type_t>
vector_t<type_t> operator *(const matrix_t<type_t>& m, const vector_t<type_t>& v)
{
    vector_t<type_t> res;

    res.x = v.x * m.x[0][0] + v.y * m.x[1][0] + v.z * m.x[2][0] + m.x[3][0];
    res.y = v.x * m.x[0][1] + v.y * m.x[1][1] + v.z * m.x[2][1] + m.x[3][1];
    res.z = v.x * m.x[0][2] + v.y * m.x[1][2] + v.z * m.x[2][2] + m.x[3][2];

    type_t denom = v.x * m.x[0][3] + v.y * m.x[1][3] + v.z * m.x[2][3] + m.x[3][3];

    // normalize
    if(denom != 1.0 && denom != 0.0)
        res /= denom;

    return res;
}

// Static methods.

template <typename type_t>
matrix_t<type_t> matrix_t<type_t>::scale(const vector_t<type_t>& v)
{
    matrix_t<type_t> res(1);
    res.x[0][0] = fabs(v.x);
    res.x[1][1] = fabs(v.y);
    res.x[2][2] = fabs(v.z);
    return res;
}

template <typename type_t>
matrix_t<type_t> matrix_t<type_t>::inv_scale(const vector_t<type_t>& v)
{
    matrix_t<type_t> res(1);
    res.x[0][0] = fabs(1.0 / v.x);
    res.x[1][1] = fabs(1.0 / v.y);
    res.x[2][2] = fabs(1.0 / v.z);
    return res;
}

template <typename type_t>
matrix_t<type_t> matrix_t<type_t>::rotate_x(type_t radians)
{
    matrix_t<type_t> res(1);
    type_t cosine = cos(radians);
    type_t sine   = sin(radians);

    res.x[1][1] = cosine;
    res.x[2][1] = -sine;
    res.x[1][2] = sine;
    res.x[2][2] = cosine;

    return res;
}

template <typename type_t>
matrix_t<type_t> matrix_t<type_t>::rotate_y(type_t radians)
{
    matrix_t<type_t> res(1);
    type_t cosine = cos(radians);
    type_t sine   = sin(radians);

    res.x[0][0] = cosine;
    res.x[2][0] = -sine;
    res.x[0][2] = sine;
    res.x[2][2] = cosine;

    return res;
}

template <typename type_t>
matrix_t<type_t> matrix_t<type_t>::rotate_z(type_t radians)
{
    matrix_t<type_t> res(1);
    type_t cosine = cos(radians);
    type_t sine   = sin(radians);

    res.x[0][0] = cosine;
    res.x[1][0] = -sine;
    res.x[0][1] = sine;
    res.x[1][1] = cosine;

    return res;
}

// Rotation for angle radians around axis
template <typename type_t>
matrix_t<type_t> matrix_t<type_t>::rotation(const vector_t<type_t>& axis, type_t radians)
{
    matrix_t<type_t> res(1);
    type_t cosine = cos(radians);
    type_t sine   = sin(radians);

    res.x[0][0] = axis.x * axis.x + (1 - axis.x * axis.x) * cosine;
    res.x[0][1] = axis.x * axis.y * (1 - cosine) + axis.z * sine;
    res.x[0][2] = axis.x * axis.z * (1 - cosine) - axis.y * sine;
    res.x[0][3] = 0;

    res.x[1][0] = axis.x * axis.y * (1 - cosine) - axis.z * sine;
    res.x[1][1] = axis.y * axis.y + (1 - axis.y * axis.y) * cosine;
    res.x[1][2] = axis.y * axis.z * (1 - cosine) + axis.x * sine;
    res.x[1][3] = 0;

    res.x[2][0] = axis.x * axis.z * (1 - cosine) + axis.y * sine;
    res.x[2][1] = axis.y * axis.z * (1 - cosine) - axis.x * sine;
    res.x[2][2] = axis.z * axis.z + (1 - axis.z * axis.z) * cosine;
    res.x[2][3] = 0;

    // not needed!
    res.x[3][0] = 0;
    res.x[3][1] = 0;
    res.x[3][2] = 0;
    res.x[3][3] = 1;

    return res;
}

// Matrix for translation by vector loc
template <typename type_t>
inline matrix_t<type_t> matrix_t<type_t>::translate(const vector_t<type_t>& loc)
{
    matrix_t res(1);
    res.x[3][0] = loc.x;
    res.x[3][1] = loc.y;
    res.x[3][2] = loc.z;
    return res;
}

// Matrix for inverse translation by vector -loc
template <typename type_t>
inline matrix_t<type_t> matrix_t<type_t>::inv_translate(const vector_t<type_t>& loc)
{
    matrix_t res(1);
    res.x[3][0] = 0.0 - loc.x;
    res.x[3][1] = 0.0 - loc.y;
    res.x[3][2] = 0.0 - loc.z;
    return res;
}

template <typename type_t>
inline matrix_t<type_t> matrix_t<type_t>::mirror_x()
{
    matrix_t<type_t> res(1);
    res.x[0][0] = -1;
    return res;
}

template <typename type_t>
inline matrix_t<type_t> matrix_t<type_t>::mirror_y()
{
    matrix_t<type_t> res(1);
    res.x[1][1] = -1;
    return res;
}

template <typename type_t>
inline matrix_t<type_t> matrix_t<type_t>::mirror_z()
{
    matrix_t<type_t> res(1);
    res.x[2][2] = -1;
    return res;
}

//
// RAII file wrapper.
//
// Copyright 2007 - 2010, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#pragma once

#include <iostream>
#include <fstream>
#include <string>
#include <arpa/inet.h>
#include <stdint.h>

namespace raii_wrapper {

class file_error
{
public:
    file_error(const char* msg_) : msg(msg_) {}
    const char* message() { return msg; }
private:
    const char* msg;//REFACTOR: use std::string
};

class file
{
    std::fstream file_;

public:
    file() {}

    file(const char* fname, std::fstream::openmode mode)
    {
        open(fname, mode);
    }

    file(const std::string& fname, std::fstream::openmode mode)
    {
        open(fname.c_str(), mode);
    }

    file(file const&) = delete;
    file& operator= (file const&) = delete;

    ~file() { close(); }

    void open(const char* fname, std::fstream::openmode mode)
    {
        file_.open(fname, mode);
        if (!file_.good())
            throw file_error("file open failure");
    }

    void close() { file_.close(); }

    void write(const void* buf, size_t count)
    {
        file_.write((const char*)buf, count);
        if (file_.bad())
            throw file_error("file write failure");
    }

    size_t read(void* buf, size_t size)
    {
        file_.read((char*)buf, size);
        return file_.gcount();
    }

    long read_pos()
    {
        return file_.tellg();
    }

    long write_pos()
    {
        return file_.tellp();
    }

    bool read_seek(long pos)
    {
        file_.seekg(pos);
        return !file_.fail();
    }

    bool write_seek(long pos)
    {
        file_.seekp(pos);
        return !file_.fail();
    }

    ssize_t size()
    {
        ssize_t old = read_pos();
        file_.seekg(0, std::fstream::end);
        ssize_t sz = read_pos();
        read_seek(old);
        return sz;
    }

    bool getline(std::string& str, char delim)
    {
        return !!std::getline(file_, str, delim);
    }

    bool getline(std::string& str)
    {
        return !!std::getline(file_, str);
    }
};

class filebinio
{
    file& file_;

public:
    filebinio(file& f) : file_(f) {}

    inline void write(const void* buf, size_t count) { file_.write(buf, count); }
    inline void write8(uint8_t datum)     { file_.write(&datum, sizeof(uint8_t));  }
    inline void write16le(uint16_t datum) { file_.write(&datum, sizeof(uint16_t)); }
    inline void write32le(uint32_t datum) { file_.write(&datum, sizeof(uint32_t)); }
    inline void write16be(uint16_t datum) { datum = htons(datum); file_.write(&datum, sizeof(uint16_t)); }
    inline void write32be(uint32_t datum) { datum = htonl(datum); file_.write(&datum, sizeof(uint32_t)); }

    inline bool read8(int8_t& datum)      { return file_.read(&datum, sizeof(int8_t)) == sizeof(int8_t); }
    inline bool read8(uint8_t& datum)     { return file_.read(&datum, sizeof(uint8_t)) == sizeof(uint8_t); }
    inline bool read16le(int16_t& datum)  { return file_.read(&datum, sizeof(int16_t)) == sizeof(int16_t); }
    inline bool read16le(uint16_t& datum) { return file_.read(&datum, sizeof(uint16_t)) == sizeof(uint16_t); }
    inline bool read32le(int32_t& datum)  { return file_.read(&datum, sizeof(int32_t)) == sizeof(int32_t); }
    inline bool read32le(uint32_t& datum) { return file_.read(&datum, sizeof(uint32_t)) == sizeof(uint32_t); }
    inline bool read16be(int16_t& datum)  { bool res = file_.read(&datum, sizeof(int16_t)) == sizeof(int16_t); datum = ntohs(datum); return res; }
    inline bool read16be(uint16_t& datum) { bool res = file_.read(&datum, sizeof(uint16_t)) == sizeof(uint16_t); datum = ntohs(datum); return res; }
    inline bool read32be(int32_t& datum)  { bool res = file_.read(&datum, sizeof(int32_t)) == sizeof(int32_t); datum = ntohl(datum); return res; }
    inline bool read32be(uint32_t& datum) { bool res = file_.read(&datum, sizeof(uint32_t)) == sizeof(uint32_t); datum = ntohl(datum); return res; }
};

// Example of using filebinio:
// filebinio& operator << (filebinio& io, string str)
// {
//     io.write((const char*)str, str.length());
//     return io;
// }

} // raii_wrapper namespace

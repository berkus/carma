#!/usr/bin/env python
# -*- coding: utf-8 -*-

VERSION = '0.0.1'
APPNAME = 'carma'

import Task, Configure
Configure.autoconfig = 1

srcdir = '.'
blddir = '__build'
source_dirs  = ['.']
include_dirs = ['.']
cflags = '-Wall -Wextra -Werror'.split()

def set_options(opts): pass

def configure(conf):
    conf.check_tool('gcc g++')

    conf.env.append_unique('CCFLAGS', cflags)
    conf.env.append_unique('CXXFLAGS', cflags)

    env = conf.env.copy()
    env.set_variant('release')
    conf.set_env_name('release', env)

    env = conf.env.copy()
    env.set_variant('debug')
    conf.set_env_name('debug', env)

    conf.setenv('release') # Activate the environment
    conf.env.append_unique('CCFLAGS', ['-O3'])
    conf.env.append_unique('CXXFLAGS', ['-O3'])

    conf.setenv('debug') # Activate the environment
    conf.env.append_unique('CCFLAGS', ['-O0', '-g'])
    conf.env.append_unique('CXXFLAGS', ['-O0', '-g'])

def build(bld):
    dat = bld.new_task_gen('cc', 'program')
    dat.source = 'dat.c'
    dat.includes = include_dirs
    dat.env = bld.env_of_name('debug').copy()
    dat.target = 'dat'

    mdl = bld.new_task_gen('cxx', 'program')
    mdl.source = 'io.cpp dump.cpp model.cpp'
    mdl.includes = include_dirs
    mdl.env = bld.env_of_name('debug').copy()
    mdl.target = 'model'

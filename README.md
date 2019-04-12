# ruckus

[![ruckus logo](https://github.com/tonal-glyph/ruckus/blob/master/logo-small.png)]

[![Build Status](https://travis-ci.org/tonal-glyph/ruckus.svg?branch=master)](https://travis-ci.org/tonal-glyph/ruckus)

rust + chuck = ruckus

A project to create a memory-safe interface for ChucK. [ChucK](https://github.com/ccrma/chuck) was created by Dr. Ge Wang of Stanford University's Center for Computer Research in Music and Acoustics (or CCRMA, pronounced "karma").  Building on the tradition of languages like CSound and SuperCollider, ChucK is a strongly-typed, strongly-timed object-oriented language. I would like for ChucK to be a first class citizen in [tgtracker](https://github.com/tonal-glyph/tgtracker).


## rust api

For a lower level interface, see the `chuck-sys` crate. [WIP]
For a higher level abstraction, see the root `ruckus` crate. [WIP]
For a small command line wrapper around the chuck binary, see the `ruck` crate.

## license

This library builds on a project licensed under GPL2. The code in this abstraction library is dual-licensed under Apache2/MIT to best serve the Rust community.
ruckus is intended to be a high-level, memory-safe abstraction over ChucK. It depends on `chuck-sys`, the low-level bindings to the lib-ified version of ChucK.
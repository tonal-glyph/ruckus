# ruckus

![ChucK logo](https://raw.githubusercontent.com/tonal-glyph/chuck-sys/master/logo-small.png)

rust + chuck = ruckus

A project to create a memory-safe interface for ChucK. ChucK was created by Dr. Ge Wang of Stanford University's Center for Computer Research in Music and Acoustics (or CCRMA, pronounced "karma"). Perry Cook and Spencer Salazar also played significant roles in its creation. Building on the tradition of languages like SuperCollider and FAUST, ChucK is a strongly-typed, strongly-timed language.

## rust api

For a lower level interface, see the [chuck-sys](https://github.com/tonal-glyph/chuck-sys) project. [WIP]

    /*
    EXAMPLE API
    io::{audio, hid, midi, osc, regex}
    ck::{ast, carry, compile, console, define, dynl, emit, err, frame, instruc, io, lang, oo, otf,
        parse, scan, shell, stats, symb, tbl, type, uana, ugen, ulib, util, vm}
    ck::uana::{xtrac, xform}
    ck::ugen::{filt, osc, stk, xxx}
    ck::ulib::{mach, math, osc, regex, std}
    ck::util::{buff, console, hid, math, net, osc, raw, serial, sndfile, string, thread, xform}
    io::osc::{addr, blob, bndl, cfg, endian, err, intrn, lowlvl, macro, msg, mthd, osctype, pmtch, send,
        serv, type, time}
    io::regex::{ast, compile, cfg, err, filter, intrn, match, mem, parse, regex, stack}
    io::regex::match::{approx, backtrack, parallel}
 
    */

## license

This library builds on a project licensed under GPL2. The code in this abstraction library is dual-licensed under Apache2/MIT to best serve the Rust community.
ruckus is intended to be a high-level, memory-safe abstraction over ChucK. It depends on [chuck_sys](https://github.com/tonal-glyph/chuck-sys). I would like for ChucK to be a first class citizen in [tgtracker](https://github.com/tonal-glyph/tgtracker).

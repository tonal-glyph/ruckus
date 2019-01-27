#![feature(maybe_uninit)]
#![feature(type_ascription)]
#![feature(untagged_unions)]
extern crate clap;
// extern crate chuck_sys;
extern crate lalrpop;
extern crate midir;
extern crate nom;
extern crate regex;
extern crate rodio;
extern crate rosc;
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
pub mod ck;
pub mod io;
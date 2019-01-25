DESTDIR?=/usr/local/bin
COREDIR=core
HOSTDIR=core
CK_PLATFORM=linux
CXXFLAGS+= -B --sysroot -DHAVE_POLL -DHAVE_LIBPTHREAD -DENABLE_THREADS -D__PLATFORM_LINUX__ -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__CK_SNDFILE_NATIVE__ -DUSE_ALSA -DUSE_DLTRICK_ALSA -DUSE_OSS -fPIC -fno-strict-aliasing -fstack-protector-strong -fno-plt -fpermissive -I. -I./core -I./core/lo -I./core/regex -I/usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include -I/usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include/c++ -O3 -mtune=generic -march=native -pipe -std=c++14
CFLAGS+= -B --sysroot -DENABLE_THREADS -D__PLATFORM_LINUX__ -D__LINUX_ALSA__ -D__UNIX_JACK__ -D__CK_SNDFILE_NATIVE__ -DUSE_ALSA -DUSE_DLTRICK_ALSA -DUSE_OSS -fPIC -fno-strict-aliasing -fstack-protector-strong -fno-plt -fpermissive -I. -I./core -I./core/lo -I./core/regex -I/usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include -I/usr/lib/gcc/x86_64-pc-linux-gnu/8.2.1/include/c++ -O3 -mtune=generic -march=native -pipe -std=c11
DL_LOAD_PATH=/usr/local/lib/chuck
LDFLAGS+= -lasound -lc -lstdc++ -lpthread -lm -lsndfile -lpulse-simple -lpulse -ldl
# LD_LIBRARY_PATH=target/debug

########################## DEFAULT MAKE TARGET #################################
# default target: print usage message and quit
current: 
	@echo "[chuck build]: please use one of the following configurations:"
	@echo "   make linux-alsa, make linux-jack, make linux-pulse,"
	@echo "   make osx, make cygwin, or make win32"


############################## MAKE INSTALL ####################################
install:
	mkdir -p $(DESTDIR)
	cp $(wildcard chuck chuck.exe) $(DESTDIR)/
	chmod 755 $(DESTDIR)/$(wildcard chuck chuck.exe)

ifneq ($(CK_TARGET),)
.DEFAULT_GOAL:=$(CK_TARGET)
ifeq ($(MAKECMDGOALS),)
MAKECMDGOALS:=$(.DEFAULT_GOAL)
endif
endif


############################## MAKE INSTALL ####################################
.PHONY: osx linux-pulse linux-jack linux-alsa cygwin osx-rl test
osx linux-pulse linux-jack linux-alsa cygwin osx-rl: chuck

win32:
	make -f $(COREDIR)/makefile.x/makefile.win32

CK_VERSION=1.4.0.0


########################### COMPILATION TOOLS ##################################
LEX=flex
YACC=bison
CC=gcc
CXX=g++
LD=g++


############################# COMPILER FLAGS ###################################
CFLAGS+=-I. -I$(COREDIR) -I$(COREDIR)/lo
CHUCK_STAT=1
ifneq ($(CHUCK_STAT),)
CFLAGS+= -D__CHUCK_STAT_TRACK__
endif

ifneq ($(CHUCK_DEBUG),)
CFLAGS+= -g
else
CFLAGS+= -O3
endif
USE_64_BIT_SAMPLE=1
ifneq ($(USE_64_BIT_SAMPLE),)
CFLAGS+= -D__CHUCK_USE_64_BIT_SAMPLE__
endif

ifneq ($(CHUCK_STRICT),)
CFLAGS+= -Wall
endif

ifneq ($(findstring arm,$(shell uname -m)),)
# some sort of arm platform- enable aggressive optimizations
CFLAGS+= -ffast-math
endif


######################### PLATFORM-SPECIFIC THINGS #############################
ifneq (,$(strip $(filter osx bin-dist-osx,$(MAKECMDGOALS))))
include $(COREDIR)/makefile.x/makefile.osx
endif

ifneq (,$(strip $(filter linux-pulse,$(MAKECMDGOALS))))
include $(COREDIR)/makefile.x/makefile.pulse
endif

ifneq (,$(strip $(filter linux-jack,$(MAKECMDGOALS))))
include $(COREDIR)/makefile.x/makefile.jack
endif

ifneq (,$(strip $(filter linux-alsa,$(MAKECMDGOALS))))
include $(COREDIR)/makefile.x/makefile.alsa
endif

ifneq (,$(strip $(filter cygwin,$(MAKECMDGOALS))))
include $(COREDIR)/makefile.x/makefile.cygwin
endif

ifneq (,$(strip $(filter osx-rl,$(MAKECMDGOALS))))
include $(COREDIR)/makefile.x/makefile.rl
endif


########################## CHUCK CORE LIB TARGETS ##############################
COBJS_CORE+= chuck.tab.o chuck.yy.o util_math.o util_network.o util_raw.o \
	util_xforms.o
CXXOBJS_CORE+= chuck.o chuck_absyn.o chuck_parse.o chuck_errmsg.o \
	chuck_frame.o chuck_symbol.o chuck_table.o chuck_utils.o \
	chuck_vm.o chuck_instr.o chuck_scan.o chuck_type.o chuck_emit.o \
	chuck_compile.o chuck_dl.o chuck_oo.o chuck_lang.o chuck_ugen.o \
	chuck_otf.o chuck_stats.o chuck_shell.o chuck_io.o chuck_carrier.o \
	hidio_sdl.o midiio_rtmidi.o rtmidi.o ugen_osc.o ugen_filter.o \
	ugen_stk.o ugen_xxx.o ulib_machine.o ulib_math.o ulib_std.o \
	ulib_opsc.o ulib_regex.o util_buffers.o util_console.o \
	util_string.o util_thread.o util_opsc.o util_serial.o \
	util_hid.o uana_xform.o uana_extract.o
LO_COBJS_CORE+= lo/address.o lo/blob.o lo/bundle.o lo/message.o lo/method.o \
	lo/pattern_match.o lo/send.o lo/server.o lo/server_thread.o lo/timetag.o


############################ CHUCK HOST TARGETS ################################
CXXSRCS_HOST+= chuck_main.cpp chuck_audio.cpp chuck_console.cpp \
	RtAudio.cpp


############################ OBJECT FILE TARGETS ###############################
CXXOBJS_HOST=$(addprefix $(HOSTDIR)/,$(CXXSRCS_HOST:.cpp=.o))

COBJS=$(COBJS_HOST) $(addprefix $(COREDIR)/,$(COBJS_CORE))
CXXOBJS=$(CXXOBJS_HOST) $(addprefix $(COREDIR)/,$(CXXOBJS_CORE))
LO_COBJS=$(addprefix $(COREDIR)/,$(LO_COBJS_CORE))
SF_COBJS=$(addprefix $(COREDIR)/,$(SF_CSRCS:.c=.o))
OBJS=$(COBJS) $(CXXOBJS) $(LO_COBJS) $(SF_COBJS)


############################ ADDITIONAL FLAGS ##################################
# for liblo headers
LO_CFLAGS=-DHAVE_CONFIG_H -I.

# remove -arch options
CFLAGSDEPEND=$(CFLAGS)

ifneq (,$(ARCHS))
ARCHOPTS=$(addprefix -arch ,$(ARCHS))
else
ARCHOPTS=
endif


############################ DISTRIBUTION INFO #################################
NOTES=AUTHORS DEVELOPER PROGRAMMER README TODO COPYING INSTALL QUICKSTART \
 THANKS VERSIONS
BIN_NOTES=README.txt
DOC_NOTES=GOTO
DIST_DIR=chuck-$(CK_VERSION)
DIST_DIR_EXE=chuck-$(CK_VERSION)-exe
CK_SVN=https://chuck-dev.stanford.edu/svn/chuck/

# pull in dependency info for *existing* .o files
-include $(OBJS:.o=.d)


############################# MAIN COMPILATION #################################
chuck-core:
	@echo -------------
	@echo [chuck-core]: compiling...
	make $(MAKECMDGOALS) -C $(COREDIR)
	@echo -------------

chuck: chuck-core $(COBJS_HOST) $(CXXOBJS_HOST)
	$(LD) -o chuck $(OBJS) $(LDFLAGS) $(ARCHOPTS)

$(COBJS_HOST): %.o: %.c
	$(CC) $(CFLAGS) $(ARCHOPTS) -c $< -o $@
	@$(CC) -MM -MQ "$@" $(CFLAGSDEPEND) $< > $*.d

$(CXXOBJS_HOST): %.o: %.cpp
	$(CXX) $(CFLAGS) $(ARCHOPTS) -c $< -o $@
	@$(CXX) -MM -MQ "$@" $(CFLAGSDEPEND) $< > $*.d

clean: 
	@rm -rf $(wildcard chuck chuck.exe) *.o *.d */*.{o,d} */*/*.{o,d} $(OBJS) \
        $(patsubst %.o,%.d,$(OBJS))*~ $(COREDIR)/chuck.output \
	$(COREDIR)/chuck.tab.h $(COREDIR)/chuck.tab.c \
        $(COREDIR)/chuck.yy.c $(DIST_DIR){,.tgz,.zip} Release Debug
	

############################### RUN TEST #######################################
test:
	pushd test; ./test.py ../chuck .; popd


############################### DISTRIBUTION ###################################
# ------------------------------------------------------------------------------
# Distribution meta-targets
# ------------------------------------------------------------------------------

.PHONY: src-dist
src-dist:
# clean out old dists
	rm -rf $(DIST_DIR) $(DIST_DIR){.tgz,.zip}
# create directories
	mkdir $(DIST_DIR) $(DIST_DIR)/src $(DIST_DIR)/examples
# copy src
	git archive HEAD src | tar -x -C $(DIST_DIR)
	rm -r $(DIST_DIR)/src/test 
# copy examples
	git archive HEAD examples | tar -x -C $(DIST_DIR)
# copy notes
	cp $(addprefix notes/,$(NOTES)) $(DIST_DIR)
# tar/gzip
	tar czf $(DIST_DIR).tgz $(DIST_DIR)
BGFLAGS= --verbose --enable-cxx-namespaces --conservative-inline-namespaces --time-phases --no-layout-tests

#derive bindings
bind:
	sed -i 's/\/\/definitions/#include "chuck_def.h"/' ../chuck/src/core/util_hid.h
	bindgen $(BGFLAGS) -o src/chuck_absyn_h.rs src/chuck_absyn.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_audio_h.rs src/chuck_audio.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_carrier_h.rs src/chuck_carrier.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_compile_h.rs src/chuck_compile.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_console_h.rs src/chuck_console.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_def_h.rs src/chuck_def.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_dl_h.rs src/chuck_dl.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_emit_h.rs src/chuck_emit.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_errmsg_h.rs src/chuck_errmsg.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_frame_h.rs src/chuck_frame.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_h.rs src/chuck.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_instr_h.rs src/chuck_instr.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_io_h.rs src/chuck_io.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_lang_h.rs src/chuck_lang.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_oo_h.rs src/chuck_oo.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_otf_h.rs src/chuck_otf.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_parse_h.rs src/chuck_parse.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_scan_h.rs src/chuck_scan.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_shell_h.rs src/chuck_shell.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_stats_h.rs src/chuck_stats.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_symbol_h.rs src/chuck_symbol.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_table_h.rs src/chuck_table.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_type_h.rs src/chuck_type.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_ugen_h.rs src/chuck_ugen.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_utils_h.rs src/chuck_utils.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/chuck_vm_h.rs src/chuck_vm.hpp -- $(CXXFLAGS)
	# bindgen $(BGFLAGS) -o src/chuck_tab_h.rs src/chuck.tab.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/hidio_sdl_h.rs src/hidio_sdl.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/lo/config_h.rs src/lo/config.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_endian_h.rs src/lo/lo_endian.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_errors_h.rs src/lo/lo_errors.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_h.rs src/lo/lo.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_internal_h.rs src/lo/lo_internal.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_lowlevel_h.rs src/lo/lo_lowlevel.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_macros_h.rs src/lo/lo_macros.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_osc_types_h.rs src/lo/lo_osc_types.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_throw_h.rs src/lo/lo_throw.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_types_h.rs src/lo/lo_types.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/lo/lo_types_internal_h.rs src/lo/lo_types_internal.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/midiio_rtmidi_h.rs src/midiio_rtmidi.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/regex/config_h.rs src/regex/config.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/regex_h.rs src/regex/regex.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_ast_h.rs src/regex/tre-ast.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_compile_h.rs src/regex/tre-compile.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_filter_h.rs src/regex/tre-filter.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_match_utils_h.rs src/regex/tre-match-utils.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/xmalloc_h.rs src/regex/xmalloc.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_config_h.rs src/regex/tre-config.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_h.rs src/regex/tre.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_internal_h.rs src/regex/tre-internal.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_match_utils_h.rs src/regex/tre-match-utils.h
	bindgen $(BGFLAGS) -o src/regex/tre_mem_h.rs src/regex/tre-mem.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_parse_h.rs src/regex/tre-parse.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/regex/tre_stack_h.rs src/regex/tre-stack.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/RtAudio_h.rs src/RtAudio.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/RtError_h.rs src/RtError.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/rtmidi_h.rs src/rtmidi.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/sndfile_h.rs src/sndfile.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/sndfile_config_h.rs src/sndfile_config.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/uana_extract_h.rs src/uana_extract.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/uana_xform_h.rs src/uana_xform.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ugen_filter_h.rs src/ugen_filter.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ugen_osc_h.rs src/ugen_osc.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ugen_stk_h.rs src/ugen_stk.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ugen_xxx_h.rs src/ugen_xxx.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ulib_machine_h.rs src/ulib_machine.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ulib_math_h.rs src/ulib_math.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ulib_opsc_h.rs src/ulib_opsc.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ulib_regex_h.rs src/ulib_regex.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/ulib_std_h.rs src/ulib_std.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_buffers_h.rs src/util_buffers.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_console_h.rs src/util_console.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_hid_h.rs src/util_hid.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_math_h.rs src/util_math.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/util_network_h.rs src/util_network.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/util_opsc_h.rs src/util_opsc.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_raw_h.rs src/util_raw.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/util_serial_h.rs src/util_serial.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_sndfile_h.rs src/util_sndfile.h -- $(CFLAGS)
	bindgen $(BGFLAGS) -o src/util_string_h.rs src/util_string.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_thread_h.rs src/util_thread.hpp -- $(CXXFLAGS)
	bindgen $(BGFLAGS) -o src/util_xforms_h.rs src/util_xforms.h -- $(CFLAGS)
	sed -i 's/pub const FP_INFINITE: u32 =;/\/\/pub const FP_INFINITE: u32;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_INT_DOWNWARD: u32 =;/\/\/pub const FP_INT_DOWNWARD: u32;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_INT_TONEAREST: u32 =;/\/\/pub const FP_INT_TONEAREST: u32;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_INT_TONEARESTFROMZERO: u32;/\/\/pub const FP_INT_TONEARESTFROMZERO: u32;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_INT_TOWARDZERO: u32;/\/\/pub const FP_INT_TOWARDZERO: u32;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_INT_UPWARD: u32 = 0;/\/\/pub const FP_INT_UPWARD: u32 = 0;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_NAN: u32 = 0;/\/\/pub const FP_NAN: u32 = 0;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_NORMAL: u32/\/\/pub const FP_NORMAL: u32/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_SUBNORMAL: u32/\/\/pub const FP_SUBNORMAL: u32/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const FP_ZERO: u32/\/\/pub const FP_ZERO: u32/' src/*h.rs src/**/*h.rs
	sed -i 's/pub const IPPORT_RESERVED: u32/\/\/pub const IPPORT_RESERVED: u32/' src/lo/*h.rs
	sed -i 's/pub type char_type = u32;/\/\/pub type char_type = u32;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub type int_type = c_int;/\/\/pub type int_type = c_int;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub type int_type = int_type;/\/\/pub type int_type = int_type;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub type int_type = wint_t;/\/\/pub type int_type = wint_t;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub type size_type = size_type;/\/\/pub type size_type = size_type;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub type size_type = usize;/\/\/pub type size_type = usize;/' src/*h.rs src/**/*h.rs
	sed -i 's/pub type std_size_type = size_type;/\/\/pub type std_size_type = size_type;/' src/*h.rs src/**/*h.rs

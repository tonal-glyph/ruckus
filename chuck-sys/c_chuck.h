/*  This file was handwritten to wrap C++ code in C to facilitate using
    ChucK from other languages through FFI.
    Andrew Prentice (c) 2019 Licensed under GPL2
*/
#ifndef C_CHUCK_H
#define C_CHUCK_H
#include <stdio.h>
#include <stdint.h>
#if defined _WIN32 || defined __CYGWIN__
    #ifdef C_CHUCK_NO_EXPORT
        #define API
    #else
        #define API __declspec(dllexport)
    #endif
    #ifndef __GNUC__
    #define snprintf sprintf_s
    #endif
#else
    #define API
#endif

#ifdef __cplusplus
    #define EXTERN extern "C"
#else
    #include <stdarg.h>
    #include <stdbool.h>
    #define EXTERN extern
#endif

#define C_CHUCK_API EXTERN API
#define CONST const

#ifdef __cplusplus
    extern "C" {
#endif
    // #ifdef ENUMS_AND_STRUCTS
    // C++ class typedef
    typedef struct ChucK ChucK;
    // C++ class constructor
    ChucK *newChucK();
    // C++ class methods
    // void ChucK_setParam(ChucK *){}
    // void ChucK_getParamInt(ChucK *){}
    // C++ class destructor
    void destroyChucK(ChucK* v);

    typedef struct Chuck_Carrier Chuck_Carrier;
    Chuck_Carrier *newChuck_Carrier();
    // void Chuck_Carrier_(Chuck_Carrier *){}
    void destroyChuck_Carrier(Chuck_Carrier* v);

    typedef struct Chuck_VM Chuck_VM;
    Chuck_VM *newChuck_VM();
    // void Chuck_VM_(Chuck_VM *){}
    void destroyChuck_VM(Chuck_VM* v);

    typedef struct Chuck_Compiler Chuck_Compiler;
    Chuck_Compiler *newChuck_Compiler();
    // void Chuck_Compiler_(Chuck_Compiler *){}
    void destroyChuck_Compiler(Chuck_Compiler* v);

    typedef struct Chuck_IO_Serial Chuck_IO_Serial;
    Chuck_IO_Serial *newChuck_IO_Serial();
    // void Chuck_IO_Serial_(Chuck_IO_Serial *){}
    void destroyChuck_IO_Serial(Chuck_IO_Serial* v);

    typedef struct HidInManager HidInManager;
    HidInManager *newHidInManager();
    // void HidInManager_(HidInManager *){}
    void destroyHidInManager(HidInManager* v);
    // #endif ENUMS_AND_STRUCTS
    C_CHUCK_API ChucK* newChuck(void);
    C_CHUCK_API void destroyChucK(ChucK* self);
#ifdef __cplusplus
    }
#endif
#endif // C_CHUCK_H

#ifndef wrapper_hpp
#define wrapper_hpp

#ifdef __cplusplus
extern "C" {
#endif

typedef enum ClipType
{
    ctIntersection,
    ctUnion,
    ctDifference,
    ctXor
} ClipType;

typedef enum PolyType
{
    ptSubject,
    ptClip
} PolyType;

typedef enum PolyFillType
{
    pftEvenOdd,
    pftNonZero,
    pftPositive,
    pftNegative
} PolyFillType;

#ifdef use_int32
typedef int cInt;
static cInt const loRange = 0x7FFF;
static cInt const hiRange = 0x7FFF;
#else
typedef signed long long cInt;
static cInt const loRange = 0x3FFFFFFF;
static cInt const hiRange = 0x3FFFFFFFFFFFFFFFLL;
typedef signed long long long64; // used by Int128 class
typedef unsigned long long ulong64;
#endif

typedef struct Polygons
{
    cInt* paths_vertices;
    int vertices_count;
    int count;
    int* paths_count;
    int* paths_sizes;
    PolyType* paths_types;
    int* paths_closed;
} Polygons;

Polygons execute(
    ClipType clip_type,
    Polygons polygons,
    PolyFillType subject_fill_type,
    PolyFillType clip_fill_type);

#ifdef __cplusplus
}
#endif

#endif // clipper_hpp

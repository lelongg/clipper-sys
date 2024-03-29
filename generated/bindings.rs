/* automatically generated by rust-bindgen 0.59.2 */

pub const ClipType_ctIntersection: ClipType = 0;
pub const ClipType_ctUnion: ClipType = 1;
pub const ClipType_ctDifference: ClipType = 2;
pub const ClipType_ctXor: ClipType = 3;
pub type ClipType = ::std::os::raw::c_uint;
pub const JoinType_jtSquare: JoinType = 0;
pub const JoinType_jtRound: JoinType = 1;
pub const JoinType_jtMiter: JoinType = 2;
pub type JoinType = ::std::os::raw::c_uint;
pub const EndType_etClosedPolygon: EndType = 0;
pub const EndType_etClosedLine: EndType = 1;
pub const EndType_etOpenButt: EndType = 2;
pub const EndType_etOpenSquare: EndType = 3;
pub const EndType_etOpenRound: EndType = 4;
pub type EndType = ::std::os::raw::c_uint;
pub const PolyType_ptSubject: PolyType = 0;
pub const PolyType_ptClip: PolyType = 1;
pub type PolyType = ::std::os::raw::c_uint;
pub const PolyFillType_pftEvenOdd: PolyFillType = 0;
pub const PolyFillType_pftNonZero: PolyFillType = 1;
pub const PolyFillType_pftPositive: PolyFillType = 2;
pub const PolyFillType_pftNegative: PolyFillType = 3;
pub type PolyFillType = ::std::os::raw::c_uint;
pub type cInt = ::std::os::raw::c_longlong;
pub type Vertice = [cInt; 2usize];
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Path {
    pub vertices: *mut Vertice,
    pub vertices_count: usize,
    pub closed: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Path() {
    assert_eq!(
        ::std::mem::size_of::<Path>(),
        24usize,
        concat!("Size of: ", stringify!(Path))
    );
    assert_eq!(
        ::std::mem::align_of::<Path>(),
        8usize,
        concat!("Alignment of ", stringify!(Path))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Path>())).vertices as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Path),
            "::",
            stringify!(vertices)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Path>())).vertices_count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Path),
            "::",
            stringify!(vertices_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Path>())).closed as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Path),
            "::",
            stringify!(closed)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Polygon {
    pub paths: *mut Path,
    pub paths_count: usize,
    pub type_: PolyType,
}
#[test]
fn bindgen_test_layout_Polygon() {
    assert_eq!(
        ::std::mem::size_of::<Polygon>(),
        24usize,
        concat!("Size of: ", stringify!(Polygon))
    );
    assert_eq!(
        ::std::mem::align_of::<Polygon>(),
        8usize,
        concat!("Alignment of ", stringify!(Polygon))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Polygon>())).paths as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Polygon),
            "::",
            stringify!(paths)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Polygon>())).paths_count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Polygon),
            "::",
            stringify!(paths_count)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Polygon>())).type_ as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(Polygon),
            "::",
            stringify!(type_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Polygons {
    pub polygons: *mut Polygon,
    pub polygons_count: usize,
}
#[test]
fn bindgen_test_layout_Polygons() {
    assert_eq!(
        ::std::mem::size_of::<Polygons>(),
        16usize,
        concat!("Size of: ", stringify!(Polygons))
    );
    assert_eq!(
        ::std::mem::align_of::<Polygons>(),
        8usize,
        concat!("Alignment of ", stringify!(Polygons))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Polygons>())).polygons as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(Polygons),
            "::",
            stringify!(polygons)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<Polygons>())).polygons_count as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(Polygons),
            "::",
            stringify!(polygons_count)
        )
    );
}
extern "C" {
    pub fn execute(
        clip_type: ClipType,
        polygons: Polygons,
        subject_fill_type: PolyFillType,
        clip_fill_type: PolyFillType,
    ) -> Polygons;
}
extern "C" {
    pub fn offset_simplify_clean(
        polygons: Polygons,
        miter_limit: f64,
        round_precision: f64,
        join_type: JoinType,
        end_type: EndType,
        delta: f64,
        fill_type: PolyFillType,
        distance: f64,
    ) -> Polygons;
}
extern "C" {
    pub fn offset(
        miter_limit: f64,
        round_precision: f64,
        join_type: JoinType,
        end_type: EndType,
        polygons: Polygons,
        delta: f64,
    ) -> Polygons;
}
extern "C" {
    pub fn simplify(polygons: Polygons, fill_type: PolyFillType) -> Polygons;
}
extern "C" {
    pub fn clean(polygons: Polygons, distance: f64) -> Polygons;
}
extern "C" {
    pub fn free_path(path: Path);
}
extern "C" {
    pub fn free_polygon(polygon: Polygon);
}
extern "C" {
    pub fn free_polygons(polygons: Polygons);
}

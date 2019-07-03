#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let clip_type = ClipType_ctUnion;
        let polygons = Polygons {
            paths_vertices: std::ptr::null_mut(),
            vertices_count: 0,
            count: 0,
            paths_count: std::ptr::null_mut(),
            paths_sizes: std::ptr::null_mut(),
            paths_types: std::ptr::null_mut(),
            paths_closed: std::ptr::null_mut(),
        };
        let subject_fill_type = PolyFillType_pftNonZero;
        let clip_fill_type = PolyFillType_pftNonZero;

        unsafe {
            execute(clip_type, polygons, subject_fill_type, clip_fill_type);
        }
    }
}

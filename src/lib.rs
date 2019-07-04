pub mod clipper {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    #![allow(dead_code)]
    #![allow(clippy::unreadable_literal)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    impl Path {
        pub fn vertices(&self) -> &[[i64; 2]] {
            unsafe { std::slice::from_raw_parts(self.vertices, self.vertices_count as usize) }
        }
    }

    impl PartialEq for Path {
        fn eq(&self, other: &Self) -> bool {
            self.closed == other.closed && self.vertices() == other.vertices()
        }
    }

    impl Eq for Path {}

    impl Polygon {
        pub fn paths(&self) -> &[Path] {
            unsafe { std::slice::from_raw_parts(self.paths, self.paths_count as usize) }
        }
    }

    impl PartialEq for Polygon {
        fn eq(&self, other: &Self) -> bool {
            self.type_ == other.type_ && self.paths() == other.paths()
        }
    }

    impl Eq for Polygon {}

    impl Polygons {
        pub fn polygons(&self) -> &[Polygon] {
            unsafe { std::slice::from_raw_parts(self.polygons, self.polygons_count as usize) }
        }
    }

    impl PartialEq for Polygons {
        fn eq(&self, other: &Self) -> bool {
            self.polygons() == other.polygons()
        }
    }

    impl Eq for Polygons {}
}

pub mod debug {
    use crate::clipper;

    pub struct PolygonsDebug<'a>(pub &'a clipper::Polygons);
    pub struct PolygonDebug<'a>(pub &'a clipper::Polygon);
    pub struct PathDebug<'a>(pub &'a clipper::Path);
    pub struct PathsDebug<'a>(pub &'a [clipper::Path]);
    pub struct PolyTypeDebug<'a>(pub &'a clipper::PolyType);
    pub struct VerticesDebug<'a>(pub &'a [clipper::Vertice]);
    pub struct VerticeDebug<'a>(pub &'a clipper::Vertice);

    impl<'a> std::fmt::Debug for PolygonsDebug<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_list()
                .entries(
                    self.0
                        .polygons()
                        .iter()
                        .map(|polygon| PolygonDebug(polygon)),
                )
                .finish()
        }
    }

    impl<'a> std::fmt::Debug for PolygonDebug<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("Polygon")
                .field("paths", &PathsDebug(self.0.paths()))
                .field("type", &PolyTypeDebug(&self.0.type_))
                .finish()
        }
    }

    impl<'a> std::fmt::Debug for PathDebug<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_struct("Path")
                .field("vertices", &VerticesDebug(self.0.vertices()))
                .field("closed", &self.0.closed)
                .finish()
        }
    }

    impl<'a> std::fmt::Debug for VerticesDebug<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_list()
                .entries(self.0.iter().map(|vertice| VerticeDebug(vertice)))
                .finish()
        }
    }

    impl<'a> std::fmt::Debug for VerticeDebug<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_list()
                .entry(&self.0[0])
                .entry(&self.0[1])
                .finish()
        }
    }

    impl<'a> std::fmt::Debug for PathsDebug<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.debug_list()
                .entries(self.0.iter().map(|path| PathDebug(path)))
                .finish()
        }
    }

    impl<'a> std::fmt::Debug for PolyTypeDebug<'a> {
        fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
            fmt.write_str(match *self.0 {
                clipper::PolyType_ptSubject => "ptSubject",
                clipper::PolyType_ptClip => "ptClip",
                _ => "invalid type",
            })
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::clipper::*;

    #[test]
    fn test_no_polygons() {
        let polygons = Polygons {
            polygons: std::ptr::null_mut(),
            polygons_count: 0,
        };
        let result = unsafe {
            execute(
                ClipType_ctUnion,
                polygons,
                PolyFillType_pftNonZero,
                PolyFillType_pftNonZero,
            )
        };
        unsafe { free_polygons(result) };
    }

    #[test]
    fn test_intersection_with_hole() {
        let polygons = Polygons {
            polygons: [
                Polygon {
                    type_: PolyType_ptSubject,
                    paths: [
                        Path {
                            vertices: [[180, 200], [260, 200], [260, 150], [180, 150]].as_mut_ptr(),
                            vertices_count: 4,
                            closed: 1,
                        },
                        Path {
                            vertices: [[215, 160], [230, 190], [200, 190]].as_mut_ptr(),
                            vertices_count: 3,
                            closed: 1,
                        },
                    ]
                    .as_mut_ptr(),
                    paths_count: 2,
                },
                Polygon {
                    type_: PolyType_ptClip,
                    paths: [Path {
                        vertices: [[190, 210], [240, 210], [240, 130], [190, 130]].as_mut_ptr(),
                        vertices_count: 4,
                        closed: 1,
                    }]
                    .as_mut_ptr(),
                    paths_count: 1,
                },
            ]
            .as_mut_ptr(),
            polygons_count: 2,
        };

        let expected = Polygons {
            polygons: [Polygon {
                type_: PolyType_ptSubject,
                paths: [
                    Path {
                        vertices: [[240, 200], [190, 200], [190, 150], [240, 150]].as_mut_ptr(),
                        vertices_count: 4,
                        closed: 1,
                    },
                    Path {
                        vertices: [[200, 190], [230, 190], [215, 160]].as_mut_ptr(),
                        vertices_count: 3,
                        closed: 1,
                    },
                ]
                .as_mut_ptr(),
                paths_count: 2,
            }]
            .as_mut_ptr(),
            polygons_count: 1,
        };

        let result = unsafe {
            execute(
                ClipType_ctIntersection,
                polygons,
                PolyFillType_pftNonZero,
                PolyFillType_pftNonZero,
            )
        };

        assert_eq!(expected, result);
        unsafe { free_polygons(result) };
    }
}
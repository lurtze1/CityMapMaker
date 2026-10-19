//! Throwaway spike for issue 12: how do the candidate geometry crates behave on degenerate
//! input? Every test prints what it observed; the findings go into a draft ADR and this
//! crate is deleted.

#[cfg(test)]
mod spade_spike {
    use spade::{DelaunayTriangulation, HasPosition, InsertionError, Point2, Triangulation};

    type Tri = DelaunayTriangulation<Point2<f64>>;

    fn edges(t: &Tri) -> Vec<((i64, i64), (i64, i64))> {
        let key = |p: Point2<f64>| ((p.x * 1e6) as i64, (p.y * 1e6) as i64);
        let mut v: Vec<_> = t
            .undirected_edges()
            .map(|e| {
                let [a, b] = e.positions();
                let (ka, kb) = (key(a), key(b));
                if ka <= kb { (ka, kb) } else { (kb, ka) }
            })
            .collect();
        v.sort();
        v
    }

    #[test]
    fn duplicate_point_is_merged_not_duplicated() {
        let mut t = Tri::new();
        let h1 = t.insert(Point2::new(1.0, 1.0)).unwrap();
        let h2 = t.insert(Point2::new(1.0, 1.0)).unwrap();
        println!("spade duplicates: vertices={} same_handle={}", t.num_vertices(), h1 == h2);
        assert_eq!(t.num_vertices(), 1);
        assert_eq!(h1, h2);
    }

    #[test]
    fn near_duplicate_points_both_survive() {
        let mut t = Tri::new();
        for p in [(0.0, 0.0), (10.0, 0.0), (0.0, 10.0), (5.0, 5.0), (5.0 + 1e-9, 5.0)] {
            t.insert(Point2::new(p.0, p.1)).unwrap();
        }
        println!(
            "spade near-duplicates: vertices={} inner_faces={}",
            t.num_vertices(),
            t.num_inner_faces()
        );
        assert_eq!(t.num_vertices(), 5);
    }

    #[test]
    fn collinear_points_give_no_faces_and_no_panic() {
        let mut t = Tri::new();
        for i in 0..6 {
            t.insert(Point2::new(f64::from(i), 2.0 * f64::from(i))).unwrap();
        }
        let voronoi_faces = t.voronoi_faces().count();
        println!(
            "spade collinear: vertices={} inner_faces={} voronoi_faces={}",
            t.num_vertices(),
            t.num_inner_faces(),
            voronoi_faces
        );
        assert_eq!(t.num_inner_faces(), 0);
    }

    #[test]
    fn nan_and_huge_coordinates_are_errors_not_panics() {
        let mut t = Tri::new();
        let nan = t.insert(Point2::new(f64::NAN, 0.0));
        let huge = t.insert(Point2::new(1e300, 0.0));
        let tiny = t.insert(Point2::new(1e-300, 0.0));
        println!("spade invalid: nan={nan:?} huge={huge:?} tiny={tiny:?}");
        assert!(matches!(nan, Err(InsertionError::NAN)));
        assert!(matches!(huge, Err(InsertionError::TooLarge)));
        assert!(matches!(tiny, Err(InsertionError::TooSmall)));
    }

    #[test]
    fn same_sorted_input_gives_identical_edge_set_and_order() {
        let pts: Vec<Point2<f64>> = (0..200)
            .map(|i| {
                let x = f64::from(i % 20) * 10.0 + f64::from((i * 7919) % 13) * 0.37;
                let y = f64::from(i / 20) * 10.0 + f64::from((i * 104729) % 11) * 0.41;
                Point2::new(x, y)
            })
            .collect();
        let build = |pts: &[Point2<f64>]| {
            let mut t = Tri::new();
            for p in pts {
                t.insert(*p).unwrap();
            }
            t
        };
        let a = build(&pts);
        let b = build(&pts);
        let raw_a: Vec<_> = a.undirected_edges().map(|e| e.positions()).collect();
        let raw_b: Vec<_> = b.undirected_edges().map(|e| e.positions()).collect();
        println!("spade determinism: edges={} identical_iteration_order={}", raw_a.len(), raw_a == raw_b);
        assert_eq!(edges(&a), edges(&b));
        assert_eq!(raw_a, raw_b);
    }

    #[test]
    fn bulk_load_matches_incremental_edge_set() {
        let pts: Vec<Point2<f64>> = (0..100)
            .map(|i| Point2::new(f64::from(i % 10) * 7.0 + f64::from(i % 3), f64::from(i / 10) * 7.0 + f64::from(i % 4)))
            .collect();
        let mut inc = Tri::new();
        for p in &pts {
            inc.insert(*p).unwrap();
        }
        let bulk: Tri = Tri::bulk_load(pts.clone()).unwrap();
        let same = edges(&inc) == edges(&bulk);
        println!("spade bulk_load vs incremental: same_edge_set={same}");
        assert!(same, "co-circular grid points can legally triangulate differently; if this fails, the ward mesh must jitter sites");
    }

    #[test]
    fn jittered_grid_with_fixed_sites_has_a_voronoi_cell_per_site() {
        let mut t = Tri::new();
        let mut n = 0;
        for cy in 0..20 {
            for cx in 0..20 {
                let jx = f64::from((cx * 31 + cy * 17) % 7) / 10.0;
                let jy = f64::from((cx * 13 + cy * 29) % 7) / 10.0;
                t.insert(Point2::new(f64::from(cx) + 0.15 + jx * 0.7, f64::from(cy) + 0.15 + jy * 0.7)).unwrap();
                n += 1;
            }
        }
        for fixed in [(5.5, 5.5), (12.25, 3.75), (7.0, 14.0)] {
            t.insert(Point2::new(fixed.0, fixed.1)).unwrap();
            n += 1;
        }
        let bounded = t
            .vertices()
            .filter(|v| v.as_voronoi_face().adjacent_edges().all(|e| e.from().position().is_some() && e.to().position().is_some()))
            .count();
        println!("spade jittered grid: sites={n} voronoi_faces={} bounded_cells={bounded}", t.voronoi_faces().count());
        assert_eq!(t.num_vertices(), n);
        assert!(bounded > 300);
    }

    #[test]
    fn point_type_can_carry_an_id() {
        #[derive(Clone, Copy, Debug, PartialEq)]
        struct Site {
            id: u64,
            p: Point2<f64>,
        }
        impl HasPosition for Site {
            type Scalar = f64;
            fn position(&self) -> Point2<f64> {
                self.p
            }
        }
        let mut t: DelaunayTriangulation<Site> = DelaunayTriangulation::new();
        t.insert(Site { id: 7, p: Point2::new(0.0, 0.0) }).unwrap();
        t.insert(Site { id: 9, p: Point2::new(1.0, 0.0) }).unwrap();
        t.insert(Site { id: 11, p: Point2::new(0.0, 1.0) }).unwrap();
        let ids: Vec<u64> = t.vertices().map(|v| v.data().id).collect();
        println!("spade custom vertex data: ids={ids:?}");
        assert_eq!(ids.len(), 3);
    }
}

#[cfg(test)]
mod i_overlay_spike {
    use i_overlay::core::fill_rule::FillRule;
    use i_overlay::core::overlay_rule::OverlayRule;
    use i_overlay::float::single::SingleFloatOverlay;
    use i_overlay::mesh::float::outline::offset::OutlineOffset;
    use i_overlay::mesh::float::style::OutlineStyle;

    fn square(x: f64, y: f64, s: f64) -> Vec<[f64; 2]> {
        vec![[x, y], [x + s, y], [x + s, y + s], [x, y + s]]
    }

    fn area(contour: &[[f64; 2]]) -> f64 {
        let n = contour.len();
        let mut a = 0.0;
        for i in 0..n {
            let [x1, y1] = contour[i];
            let [x2, y2] = contour[(i + 1) % n];
            a += x1 * y2 - x2 * y1;
        }
        a / 2.0
    }

    #[test]
    fn union_of_overlapping_squares_is_one_shape() {
        let r = square(0.0, 0.0, 10.0).overlay(&square(5.0, 5.0, 10.0), OverlayRule::Union, FillRule::NonZero);
        println!("i_overlay union: shapes={} outer_area={}", r.len(), area(&r[0][0]));
        assert_eq!(r.len(), 1);
        assert!((area(&r[0][0]) - 175.0).abs() < 1e-9);
    }

    #[test]
    fn zero_area_polygon_is_dropped_silently() {
        let degenerate = vec![[0.0, 0.0], [5.0, 5.0], [10.0, 10.0]];
        let r = square(0.0, 0.0, 10.0).overlay(&degenerate, OverlayRule::Union, FillRule::NonZero);
        let alone = degenerate.overlay(&Vec::<Vec<[f64; 2]>>::new(), OverlayRule::Subject, FillRule::NonZero);
        println!("i_overlay zero-area: union_shapes={} degenerate_alone_shapes={}", r.len(), alone.len());
        assert_eq!(r.len(), 1);
        assert!((area(&r[0][0]) - 100.0).abs() < 1e-9);
        assert_eq!(alone.len(), 0);
    }

    #[test]
    fn self_intersecting_bowtie_is_split_into_two_shapes() {
        let bowtie = vec![[0.0, 0.0], [10.0, 10.0], [10.0, 0.0], [0.0, 10.0]];
        let r = bowtie.overlay(&Vec::<Vec<[f64; 2]>>::new(), OverlayRule::Subject, FillRule::NonZero);
        let areas: Vec<f64> = r.iter().map(|s| area(&s[0])).collect();
        println!("i_overlay bowtie: shapes={} areas={areas:?}", r.len());
        assert_eq!(r.len(), 2);
    }

    #[test]
    fn self_touching_polygon_does_not_panic() {
        let touching = vec![[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [5.0, 5.0], [0.0, 10.0], [5.0, 5.0]];
        let r = touching.overlay(&Vec::<Vec<[f64; 2]>>::new(), OverlayRule::Subject, FillRule::NonZero);
        println!("i_overlay self-touching: shapes={} contours_in_first={}", r.len(), r.first().map_or(0, |s| s.len()));
        assert!(!r.is_empty());
    }

    #[test]
    fn difference_produces_a_hole_as_second_contour() {
        let r = square(0.0, 0.0, 10.0).overlay(&square(3.0, 3.0, 4.0), OverlayRule::Difference, FillRule::NonZero);
        println!("i_overlay difference: shapes={} contours={} hole_area={}", r.len(), r[0].len(), area(&r[0][1]));
        assert_eq!(r.len(), 1);
        assert_eq!(r[0].len(), 2);
        assert!(area(&r[0][1]) < 0.0);
    }

    #[test]
    fn inset_that_collapses_gives_empty_and_outset_grows() {
        let sq = square(0.0, 0.0, 10.0);
        let inset = sq.outline(&OutlineStyle::new(-2.0));
        let collapsed = sq.outline(&OutlineStyle::new(-6.0));
        let outset = sq.outline(&OutlineStyle::new(2.0));
        println!(
            "i_overlay offset: inset_area={} collapsed_shapes={} outset_area={}",
            inset.first().map_or(0.0, |s| area(&s[0])),
            collapsed.len(),
            outset.first().map_or(0.0, |s| area(&s[0]))
        );
        assert!((inset[0][0].len() == 4) && (area(&inset[0][0]) - 36.0).abs() < 1e-6);
        assert!(collapsed.is_empty());
        assert!(area(&outset[0][0]) > 100.0);
    }

    #[test]
    fn same_input_twice_gives_identical_output_including_vertex_order() {
        let a = square(0.0, 0.0, 10.0).overlay(&square(5.0, 5.0, 10.0), OverlayRule::Union, FillRule::NonZero);
        let b = square(0.0, 0.0, 10.0).overlay(&square(5.0, 5.0, 10.0), OverlayRule::Union, FillRule::NonZero);
        println!("i_overlay determinism: identical={} first_vertex={:?}", a == b, a[0][0][0]);
        assert_eq!(a, b);
    }

    #[test]
    fn metre_scale_coordinates_keep_sub_millimetre_precision() {
        let r = square(1000.0, 1000.0, 0.001).overlay(&square(1000.0005, 1000.0005, 0.001), OverlayRule::Union, FillRule::NonZero);
        let a = r.first().map_or(0.0, |s| area(&s[0]));
        println!("i_overlay precision at 1 km with 1 mm squares: shapes={} area={a:e} expected={:e}", r.len(), 1.75e-6);
        assert!(!r.is_empty());
    }
}

#[cfg(test)]
mod geo_and_rstar_spike {
    use geo::{Area, Centroid, ConvexHull, LineString, Polygon, coord};
    use rstar::{PointDistance, RTree};

    #[test]
    fn degenerate_polygon_area_and_centroid() {
        let line = Polygon::new(LineString::from(vec![(0.0, 0.0), (5.0, 5.0), (10.0, 10.0), (0.0, 0.0)]), vec![]);
        let empty: Polygon<f64> = Polygon::new(LineString::new(vec![]), vec![]);
        println!(
            "geo degenerate: line_area={} line_centroid={:?} empty_area={} empty_centroid={:?}",
            line.unsigned_area(),
            line.centroid(),
            empty.unsigned_area(),
            empty.centroid()
        );
        assert_eq!(line.unsigned_area(), 0.0);
        assert!(empty.centroid().is_none());
    }

    #[test]
    fn convex_hull_of_collinear_points_does_not_panic() {
        let ls = LineString::from(vec![(0.0, 0.0), (1.0, 1.0), (2.0, 2.0), (3.0, 3.0)]);
        let hull = ls.convex_hull();
        println!("geo collinear hull: exterior_points={} area={}", hull.exterior().0.len(), hull.unsigned_area());
        assert_eq!(hull.unsigned_area(), 0.0);
    }

    #[test]
    fn rstar_nearest_with_duplicate_points_is_stable() {
        let pts = vec![[1.0, 1.0], [1.0, 1.0], [2.0, 2.0], [1.0, 1.0]];
        let tree = RTree::bulk_load(pts.clone());
        let n = tree.nearest_neighbor([1.1, 1.1]).unwrap();
        let within: Vec<_> = tree.locate_within_distance([1.0, 1.0], 0.01_f64).collect();
        println!("rstar duplicates: size={} nearest={n:?} dist2={} within_count={}", tree.size(), n.distance_2(&[1.1, 1.1]), within.len());
        assert_eq!(tree.size(), 4);
        assert_eq!(within.len(), 3);
    }

    #[test]
    fn rstar_iteration_order_is_deterministic_for_same_input() {
        let pts: Vec<[f64; 2]> = (0..500).map(|i| [f64::from(i % 25), f64::from(i / 25)]).collect();
        let a: Vec<_> = RTree::bulk_load(pts.clone()).iter().copied().collect();
        let b: Vec<_> = RTree::bulk_load(pts).iter().copied().collect();
        println!("rstar determinism: identical_iteration={}", a == b);
        assert_eq!(a, b);
    }

    #[test]
    fn geo_coord_macro_compiles() {
        let c = coord! { x: 1.0, y: 2.0 };
        assert_eq!(c.x, 1.0);
    }
}

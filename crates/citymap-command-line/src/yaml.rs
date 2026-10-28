//! The YAML facade: the only module that calls `serde_saphyr` (ADR-034).
//! Every other module reads and writes YAML through `from_text` and `to_text`, so the YAML
//! crate can be replaced here alone.

use serde::Serialize;
use serde::de::DeserializeOwned;

/// A place in YAML text: line and column, both counted from 1.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TextPosition {
    /// The line, counted from 1.
    pub line: u64,
    /// The column, counted from 1.
    pub column: u64,
}

/// Why YAML text could not become a value, or a value could not become YAML text. It holds
/// the YAML crate's message and position as our own values, never the crate's error type.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum YamlError {
    /// The text is not valid YAML or does not match the expected type.
    #[error("{message}")]
    Reading {
        /// What went wrong, as the YAML crate words it, with a snippet of the text.
        message: String,
        /// Where in the text it went wrong; `None` when the YAML crate gives no place.
        position: Option<TextPosition>,
    },
    /// The value could not be written as YAML.
    #[error("{message}")]
    Writing {
        /// What went wrong, as the YAML crate words it.
        message: String,
    },
}

/// Reads one value of type `T` from YAML text; the error names the line and column.
///
/// # Errors
///
/// Returns `YamlError::Reading` when the text is not valid YAML or does not match `T`, for
/// example an unknown key, a duplicate key, a missing field or an unknown enum value.
pub fn from_text<T: DeserializeOwned>(text: &str) -> Result<T, YamlError> {
    serde_saphyr::from_str(text).map_err(|error| YamlError::Reading {
        message: error.to_string(),
        position: error.location().map(|location| TextPosition {
            line: location.line(),
            column: location.column(),
        }),
    })
}

/// Writes one value as YAML text that `from_text` reads back to an equal value.
///
/// # Errors
///
/// Returns `YamlError::Writing` when the value cannot be written as YAML.
pub fn to_text<T: Serialize>(value: &T) -> Result<String, YamlError> {
    serde_saphyr::to_string(value).map_err(|error| YamlError::Writing {
        message: error.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use citymap_core::random::Seed;
    use citymap_core::specification::{
        Band, Bearing, Canal, Cliff, Coast, CoastShape, CompassRegion, Era, Harbour, Hill, Hollow,
        Identity, Importance, InsideCover, Lake, LandCover, OutsideCover, Plateau, Presets, River,
        RiverCrossingKind, Road, RoadEnd, Settlement, Shore, Site, Slope, Specification, Terrain,
        Water,
    };
    use citymap_core::units::{Gradient, Length};
    use proptest::prelude::*;

    use super::{TextPosition, YamlError, from_text, to_text};

    const HEADER: &str = "\
schema: 1
identity:
  name: Harrowmere
  seed: 20261019
  notes: Fictional river port used by the test suite.
settlement:
  population: 6000
site:
  extent_metres: 2400
  water:
    rivers:
      - identifier: ash
        from: north_west
        to: east
        width_metres: 18
roads:
  - identifier: north_road
    ends: [north, south]
    importance: major
    river_crossing: ford
  - identifier: west_road
    ends: [west, core]
    importance: minor
presets:
  era: early_medieval
";

    fn harrowmere() -> Specification {
        Specification {
            schema: 1,
            identity: Identity {
                name: "Harrowmere".to_string(),
                seed: Seed::new(20_261_019),
                notes: "Fictional river port used by the test suite.".to_string(),
            },
            settlement: Settlement { population: 6000 },
            site: Site {
                extent_metres: length(2400.0),
                terrain: Terrain::default(),
                land_cover: LandCover::default(),
                water: Water {
                    rivers: vec![River {
                        identifier: "ash".to_string(),
                        from: CompassRegion::NorthWest,
                        to: CompassRegion::East,
                        width_metres: length(18.0),
                    }],
                    ..Water::default()
                },
            },
            roads: vec![
                Road {
                    identifier: "north_road".to_string(),
                    ends: [
                        RoadEnd::Region(CompassRegion::North),
                        RoadEnd::Region(CompassRegion::South),
                    ],
                    importance: Importance::Major,
                    river_crossing: RiverCrossingKind::Ford,
                },
                Road {
                    identifier: "west_road".to_string(),
                    ends: [RoadEnd::Region(CompassRegion::West), RoadEnd::Core],
                    importance: Importance::Minor,
                    river_crossing: RiverCrossingKind::Bridge,
                },
            ],
            presets: Presets {
                era: Era::EarlyMedieval,
            },
        }
    }

    fn length(value: f64) -> Length {
        Length::new(value).expect("test input is a valid non-negative length")
    }

    fn read(text: &str) -> Result<Specification, YamlError> {
        from_text(text)
    }

    #[test]
    fn the_example_header_reads_to_the_expected_value() {
        assert_eq!(read(HEADER), Ok(harrowmere()));
    }

    #[test]
    fn notes_default_to_empty() {
        let text = HEADER.replace(
            "  notes: Fictional river port used by the test suite.\n",
            "",
        );
        let specification = read(&text).expect("a header without notes is valid");
        assert_eq!(specification.identity.notes, "");
    }

    #[test]
    fn an_unknown_key_is_rejected_at_every_level() {
        let cases = [
            ("schema: 1\n", "schema: 1\ncolour: red\n"),
            (
                "  name: Harrowmere\n",
                "  name: Harrowmere\n  colour: red\n",
            ),
            (
                "  population: 6000\n",
                "  population: 6000\n  colour: red\n",
            ),
            (
                "  era: early_medieval\n",
                "  era: early_medieval\n  colour: red\n",
            ),
            (
                "  extent_metres: 2400\n",
                "  extent_metres: 2400\n  colour: red\n",
            ),
            ("    rivers:\n", "    colour: red\n    rivers:\n"),
            (
                "        width_metres: 18\n",
                "        width_metres: 18\n        colour: red\n",
            ),
            (
                "    importance: major\n",
                "    importance: major\n    colour: red\n",
            ),
        ];
        for (line, with_unknown_key) in cases {
            let text = HEADER.replace(line, with_unknown_key);
            let error = read(&text).expect_err("an unknown key is an error");
            assert!(
                error.to_string().contains("unknown field `colour`"),
                "{error}"
            );
        }
    }

    #[test]
    fn a_reading_error_carries_line_and_column() {
        let text = HEADER.replace(
            "schema: 1
",
            "schema: 1
colour: red
",
        );
        let error = read(&text).expect_err("an unknown key is an error");
        let YamlError::Reading { position, .. } = error else {
            panic!("reading text gives a reading error, got {error:?}");
        };
        assert_eq!(position, Some(TextPosition { line: 2, column: 1 }));
    }

    #[test]
    fn a_missing_required_field_is_rejected() {
        let lines = [
            "schema: 1\n",
            "  name: Harrowmere\n",
            "  seed: 20261019\n",
            "  population: 6000\n",
            "  extent_metres: 2400\n",
            "        from: north_west\n",
            "        width_metres: 18\n",
            "  - identifier: north_road\n",
            "    ends: [north, south]\n",
            "    importance: major\n",
            "  era: early_medieval\n",
        ];
        for line in lines {
            let text = HEADER.replace(line, "");
            assert!(read(&text).is_err(), "removing {line:?} must fail");
        }
    }

    #[test]
    fn a_duplicate_key_is_rejected() {
        let text = HEADER.replace(
            "  population: 6000\n",
            "  population: 6000\n  population: 9\n",
        );
        assert!(read(&text).is_err());
    }

    #[test]
    fn an_unknown_era_is_rejected() {
        let text = HEADER.replace("early_medieval", "bronze_age");
        assert!(read(&text).is_err());
    }

    #[test]
    fn a_negative_river_width_is_rejected() {
        let text = HEADER.replace("width_metres: 18", "width_metres: -18");
        let error = read(&text).expect_err("a negative width is an error");
        assert!(error.to_string().contains("length is negative"), "{error}");
    }

    #[test]
    fn a_band_of_an_inside_cover_is_rejected() {
        let text = HEADER.replace(
            "  water:\n",
            "  land_cover:\n    bands: [{ cover: gardens, bearing_degrees: north, depth_metres: 100 }]\n  water:\n",
        );
        let error = read(&text).expect_err("gardens are an inside cover");
        assert!(error.to_string().contains("gardens"), "{error}");
    }

    #[test]
    fn named_covers_and_bands_are_read_together() {
        let text = HEADER.replace(
            "  water:\n",
            "  land_cover:\n    outside: [forest]\n    bands: [{ cover: forest, bearing_degrees: north, depth_metres: 300 }]\n  water:\n",
        );
        let land_cover = read(&text).expect("both forms are read").site.land_cover;
        assert_eq!(land_cover.outside, [OutsideCover::Forest]);
        assert_eq!(land_cover.bands[0].cover, OutsideCover::Forest);
    }

    fn with_terrain(terrain: &str) -> String {
        HEADER.replace("  water:\n", &format!("  terrain:\n{terrain}  water:\n"))
    }

    #[test]
    fn a_bearing_is_whole_degrees_or_a_compass_word() {
        let text = with_terrain(concat!(
            "    cliffs:\n",
            "      - { bearing_degrees: 115, height_metres: 40 }\n",
            "      - { bearing_degrees: south_east, height_metres: 40 }\n",
        ));
        let specification = read(&text).expect("both forms are read");
        let bearings: Vec<Bearing> = specification
            .site
            .terrain
            .cliffs
            .iter()
            .map(|cliff| cliff.bearing_degrees)
            .collect();
        assert_eq!(
            bearings,
            [
                Bearing::Degrees(115),
                Bearing::Word(CompassRegion::SouthEast)
            ]
        );
    }

    #[test]
    fn a_compass_word_is_written_back_as_the_word() {
        let text =
            with_terrain("    slope: { toward_degrees: south_east, metres_per_kilometre: 15 }\n");
        let specification = read(&text).expect("the slope is read");
        let written = to_text(&specification).expect("a specification always writes");
        assert!(written.contains("toward_degrees: south_east"), "{written}");
    }

    #[test]
    fn a_bearing_that_is_neither_degrees_nor_a_word_is_rejected() {
        for bearing in ["sideways", "-5", "12.5", "0x10", "+5", "007"] {
            let text = with_terrain(&format!(
                "    plateau: {{ bearing_degrees: {bearing}, height_metres: 30 }}\n"
            ));
            let error = read(&text).expect_err("the bearing is rejected");
            assert!(
                error.to_string().contains("a bearing: whole degrees"),
                "{error}"
            );
        }
    }

    #[test]
    fn the_old_side_key_of_the_proposal_is_rejected() {
        let text = with_terrain("    coast: { side: east, shape: bay }\n");
        let error = read(&text).expect_err("`side` is not a terrain key");
        assert!(
            error.to_string().contains("unknown field `side`"),
            "{error}"
        );
    }

    #[test]
    fn a_coast_without_shore_leaves_the_shore_to_the_generator() {
        let text = with_terrain("    coast: { bearing_degrees: east, shape: bay }\n");
        let specification = read(&text).expect("the coast is read");
        let coast = specification
            .site
            .terrain
            .coast
            .expect("the file has a coast");
        assert_eq!((coast.shape, coast.shore), (CoastShape::Bay, None));
    }

    #[test]
    fn water_without_tidal_is_not_tidal() {
        let text = HEADER.replace(
            "  water:\n",
            "  water:\n    lakes: [{ identifier: mere, at: south_west, radius_metres: 150 }]\n",
        );
        let specification = read(&text).expect("the lake is read");
        assert_eq!(specification.site.water.lakes.len(), 1);
        assert!(!specification.site.water.tidal);
    }

    #[test]
    fn a_site_without_terrain_has_none() {
        let specification = read(HEADER).expect("terrain is optional");
        assert_eq!(specification.site.terrain, Terrain::default());
    }

    #[test]
    fn a_site_without_water_has_no_rivers() {
        let start = HEADER.find("  water:\n").expect("the header has water");
        let end = HEADER.find("roads:").expect("the header has roads");
        let text = format!("{}{}", &HEADER[..start], &HEADER[end..]);
        let specification = read(&text).expect("water is optional");
        assert_eq!(specification.site.water.rivers, Vec::<River>::new());
    }

    #[test]
    fn an_unknown_compass_region_is_rejected() {
        let text = HEADER.replace("from: north_west", "from: up_river");
        assert!(read(&text).is_err());
    }

    #[test]
    fn a_road_without_river_crossing_crosses_by_bridge() {
        let specification = read(HEADER).expect("the header is valid");
        assert_eq!(
            specification.roads[1].river_crossing,
            RiverCrossingKind::Bridge
        );
    }

    #[test]
    fn a_landmark_end_is_read_as_a_place() {
        let text = HEADER.replace("ends: [west, core]", "ends: [west, guildhall]");
        let specification = read(&text).expect("a place end reads; validation rejects it");
        assert_eq!(
            specification.roads[1].ends[1],
            RoadEnd::Place("guildhall".to_string())
        );
    }

    #[test]
    fn an_unknown_importance_or_crossing_kind_is_rejected() {
        for (from, to) in [
            ("importance: major", "importance: huge"),
            ("river_crossing: ford", "river_crossing: ferry"),
        ] {
            let text = HEADER.replace(from, to);
            assert!(read(&text).is_err(), "{to} must fail");
        }
    }

    #[test]
    fn a_centre_end_is_read_as_a_region() {
        let text = HEADER.replace("ends: [west, core]", "ends: [west, centre]");
        let specification = read(&text).expect("centre reads; validation rejects it");
        assert_eq!(
            specification.roads[1].ends[1],
            RoadEnd::Region(CompassRegion::Centre)
        );
    }

    #[test]
    fn a_road_has_exactly_two_ends() {
        for ends in ["ends: [west]", "ends: [west, core, east]"] {
            let text = HEADER.replace("ends: [west, core]", ends);
            assert!(read(&text).is_err(), "{ends} must fail");
        }
    }

    #[test]
    fn roads_are_required() {
        let start = HEADER.find("roads:").expect("the header has roads");
        let end = HEADER.find("presets:").expect("the header has presets");
        let text = format!("{}{}", &HEADER[..start], &HEADER[end..]);
        let error = read(&text).expect_err("roads are required");
        assert!(error.to_string().contains("roads"), "{error}");
    }

    fn any_era() -> impl Strategy<Value = Era> {
        prop_oneof![
            Just(Era::EarlyMedieval),
            Just(Era::HighMedieval),
            Just(Era::LateMedieval),
        ]
    }

    fn any_compass_region() -> impl Strategy<Value = CompassRegion> {
        prop::sample::select(CompassRegion::ALL.to_vec())
    }

    fn any_road_end() -> impl Strategy<Value = RoadEnd> {
        prop_oneof![
            any_compass_region().prop_map(RoadEnd::Region),
            Just(RoadEnd::Core),
            "place_[a-z]{1,8}".prop_map(RoadEnd::Place),
        ]
    }

    fn any_road() -> impl Strategy<Value = Road> {
        (
            any::<String>(),
            any_road_end(),
            any_road_end(),
            prop_oneof![Just(Importance::Minor), Just(Importance::Major)],
            prop_oneof![
                Just(RiverCrossingKind::Bridge),
                Just(RiverCrossingKind::Ford)
            ],
        )
            .prop_map(
                |(identifier, first, second, importance, river_crossing)| Road {
                    identifier,
                    ends: [first, second],
                    importance,
                    river_crossing,
                },
            )
    }

    fn any_length() -> impl Strategy<Value = Length> {
        (0.0..=1e6_f64).prop_map(length)
    }

    fn any_river() -> impl Strategy<Value = River> {
        (
            any::<String>(),
            any_compass_region(),
            any_compass_region(),
            any_length(),
        )
            .prop_map(|(identifier, from, to, width_metres)| River {
                identifier,
                from,
                to,
                width_metres,
            })
    }

    fn any_gradient() -> impl Strategy<Value = Gradient> {
        (0.0..=1e6_f64).prop_map(|value| Gradient::new(value).expect("a valid gradient"))
    }

    fn any_bearing() -> impl Strategy<Value = Bearing> {
        prop_oneof![
            any::<u16>().prop_map(Bearing::Degrees),
            any_compass_region().prop_map(Bearing::Word),
        ]
    }

    fn any_coast() -> impl Strategy<Value = Coast> {
        (
            any_bearing(),
            prop::sample::select(vec![
                CoastShape::Bay,
                CoastShape::Open,
                CoastShape::Strait,
                CoastShape::Estuary,
            ]),
            prop::option::of(prop::sample::select(vec![
                Shore::Beach,
                Shore::Rock,
                Shore::SaltMarsh,
            ])),
        )
            .prop_map(|(bearing_degrees, shape, shore)| Coast {
                bearing_degrees,
                shape,
                shore,
            })
    }

    fn any_terrain() -> impl Strategy<Value = Terrain> {
        let cliff =
            (any_bearing(), any_length()).prop_map(|(bearing_degrees, height_metres)| Cliff {
                bearing_degrees,
                height_metres,
            });
        let hill = (any_compass_region(), any_length(), any_length()).prop_map(
            |(at, radius_metres, height_metres)| Hill {
                at,
                radius_metres,
                height_metres,
            },
        );
        let hollow = (any_compass_region(), any_length(), any_length()).prop_map(
            |(at, radius_metres, depth_metres)| Hollow {
                at,
                radius_metres,
                depth_metres,
            },
        );
        let slope =
            (any_bearing(), any_gradient()).prop_map(|(toward_degrees, metres_per_kilometre)| {
                Slope {
                    toward_degrees,
                    metres_per_kilometre,
                }
            });
        let plateau =
            (any_bearing(), any_length()).prop_map(|(bearing_degrees, height_metres)| Plateau {
                bearing_degrees,
                height_metres,
            });
        (
            prop::option::of(any_coast()),
            prop::collection::vec(cliff, 0..3),
            prop::collection::vec(hill, 0..3),
            prop::collection::vec(hollow, 0..3),
            prop::option::of(slope),
            prop::option::of(plateau),
        )
            .prop_map(|(coast, cliffs, hills, hollows, slope, plateau)| Terrain {
                coast,
                cliffs,
                hills,
                hollows,
                slope,
                plateau,
            })
    }

    fn any_water() -> impl Strategy<Value = Water> {
        let lake = (any::<String>(), any_compass_region(), any_length()).prop_map(
            |(identifier, at, radius_metres)| Lake {
                identifier,
                at,
                radius_metres,
            },
        );
        let harbour = (any::<String>(), any_bearing(), any_length()).prop_map(
            |(identifier, bearing_degrees, length_metres)| Harbour {
                identifier,
                bearing_degrees,
                length_metres,
            },
        );
        let canal = any_river().prop_map(|river| Canal {
            identifier: river.identifier,
            from: river.from,
            to: river.to,
            width_metres: river.width_metres,
        });
        (
            prop::collection::vec(any_river(), 0..4),
            prop::collection::vec(lake, 0..3),
            prop::option::of(harbour),
            any::<bool>(),
            prop::collection::vec(canal, 0..3),
        )
            .prop_map(|(rivers, lakes, harbour, tidal, canals)| Water {
                rivers,
                lakes,
                harbour,
                tidal,
                canals,
            })
    }

    fn any_land_cover() -> impl Strategy<Value = LandCover> {
        let outside = prop::sample::select(vec![
            OutsideCover::Farmland,
            OutsideCover::Orchards,
            OutsideCover::Heath,
            OutsideCover::Rock,
            OutsideCover::Marsh,
            OutsideCover::Forest,
        ]);
        let inside = prop::sample::select(vec![
            InsideCover::Gardens,
            InsideCover::Parks,
            InsideCover::Groves,
        ]);
        let band = (outside.clone(), any_bearing(), any_length()).prop_map(
            |(cover, bearing_degrees, depth_metres)| Band {
                cover,
                bearing_degrees,
                depth_metres,
            },
        );
        (
            prop::collection::vec(outside, 0..4),
            prop::collection::vec(inside, 0..3),
            prop::collection::vec(band, 0..3),
        )
            .prop_map(|(outside, inside, bands)| LandCover {
                outside,
                inside,
                bands,
            })
    }

    fn any_site() -> impl Strategy<Value = Site> {
        (any_length(), any_terrain(), any_water(), any_land_cover()).prop_map(
            |(extent_metres, terrain, water, land_cover)| Site {
                extent_metres,
                terrain,
                water,
                land_cover,
            },
        )
    }

    fn any_specification() -> impl Strategy<Value = Specification> {
        (
            any::<u32>(),
            any::<String>(),
            any::<u64>(),
            any::<String>(),
            any::<u32>(),
            any_site(),
            prop::collection::vec(any_road(), 1..4),
            any_era(),
        )
            .prop_map(
                |(schema, name, seed, notes, population, site, roads, era)| Specification {
                    schema,
                    identity: Identity {
                        name,
                        seed: Seed::new(seed),
                        notes,
                    },
                    settlement: Settlement { population },
                    site,
                    roads,
                    presets: Presets { era },
                },
            )
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(64))]

        #[test]
        fn a_specification_survives_yaml_and_back(specification in any_specification()) {
            let text = to_text(&specification).expect("a specification always writes");
            prop_assert_eq!(read(&text), Ok(specification));
        }
    }
}

//! Builders of specification values that the test modules share.

use proptest::prelude::*;

use citymap_specification::specification::{
    AlongWater, Band, Bearing, Canal, Citadel, Cliff, Coast, CoastShape, CompassRegion, Constraint,
    Crossing, Defences, District, DistrictFunction, Enclosure, Era, Footprint, FootprintClass,
    Gate, GateKind, Harbour, Hill, Hollow, Identity, Importance, InsideCover, Jurisdiction, Lake,
    LandCover, Landmark, LandmarkFunction, OpenSpace, OpenSpaceKind, OutsideCover, Placement,
    Plateau, Presets, River, RiverCrossingKind, Road, RoadEnd, Rule, Seed, Settlement, Site, Slope,
    Specification, Terrain, Wall, WallKind, Water, Wealth,
};
use citymap_specification::units::{Gradient, Length, Rotation, Weight};
use citymap_specification::validation::POPULATION;

pub fn length(value: f64) -> Length {
    Length::new(value).expect("test input is a valid length")
}

pub fn gradient(metres_per_kilometre: f64) -> Gradient {
    Gradient::new(metres_per_kilometre).expect("test input is a valid gradient")
}

pub fn river(identifier: &str, from: CompassRegion, to: CompassRegion) -> River {
    River {
        identifier: identifier.to_string(),
        from,
        to,
        width_metres: length(18.0),
    }
}

pub fn road(identifier: &str, first: RoadEnd, second: RoadEnd) -> Road {
    Road {
        identifier: identifier.to_string(),
        ends: [first, second],
        importance: Importance::Major,
        river_crossing: RiverCrossingKind::Bridge,
    }
}

pub fn valid() -> Specification {
    Specification {
        schema: 1,
        identity: Identity {
            name: "Harrowmere".to_string(),
            seed: Seed::new(1),
            notes: String::new(),
        },
        settlement: Settlement { population: 6000 },
        site: Site {
            extent_metres: length(2400.0),
            terrain: Terrain::default(),
            land_cover: LandCover::default(),
            water: Water {
                rivers: vec![river("ash", CompassRegion::NorthWest, CompassRegion::East)],
                ..Water::default()
            },
        },
        districts: vec![District {
            identifier: "old_town".to_string(),
            name: "Old Town".to_string(),
            function: DistrictFunction::Residential,
            wealth: Wealth::Middling,
            group: None,
            enclosure: Enclosure::Open,
            jurisdiction: Jurisdiction::Town,
            weight: Weight::DEFAULT,
            constraints: Vec::new(),
            notes: String::new(),
        }],
        landmarks: Vec::new(),
        defences: Defences::default(),
        open_space: Vec::new(),
        crossings: Vec::new(),
        roads: vec![
            road(
                "north_road",
                RoadEnd::Region(CompassRegion::North),
                RoadEnd::Region(CompassRegion::South),
            ),
            road(
                "west_road",
                RoadEnd::Region(CompassRegion::West),
                RoadEnd::Core,
            ),
        ],
        presets: Presets::new(Era::EarlyMedieval),
    }
}

pub fn terrain() -> Terrain {
    Terrain {
        coast: Some(Coast {
            bearing_degrees: Bearing::Degrees(100),
            shape: CoastShape::Estuary,
            shore: None,
        }),
        cliffs: vec![Cliff {
            bearing_degrees: Bearing::Word(CompassRegion::West),
            height_metres: length(40.0),
        }],
        hills: vec![Hill {
            at: CompassRegion::NorthWest,
            radius_metres: length(300.0),
            height_metres: length(60.0),
        }],
        hollows: vec![Hollow {
            at: CompassRegion::Centre,
            radius_metres: length(120.0),
            depth_metres: length(8.0),
        }],
        slope: Some(Slope {
            toward_degrees: Bearing::Word(CompassRegion::East),
            metres_per_kilometre: gradient(15.0),
        }),
        plateau: Some(Plateau {
            bearing_degrees: Bearing::Degrees(250),
            height_metres: length(30.0),
        }),
    }
}

pub fn with_terrain() -> Specification {
    let mut specification = valid();
    specification.site.terrain = terrain();
    specification
}

pub fn with_water() -> Specification {
    let mut specification = valid();
    let water = &mut specification.site.water;
    water.lakes.push(Lake {
        identifier: "mere".to_string(),
        at: CompassRegion::SouthWest,
        radius_metres: length(150.0),
    });
    water.harbour = Some(Harbour {
        identifier: "quay".to_string(),
        bearing_degrees: Bearing::Degrees(60),
        length_metres: length(200.0),
    });
    water.tidal = true;
    water.canals.push(Canal {
        identifier: "cut".to_string(),
        from: CompassRegion::South,
        to: CompassRegion::West,
        width_metres: length(6.0),
    });
    specification
}

pub fn harbour_at(degrees: u16) -> Specification {
    let mut specification = with_water();
    if let Some(harbour) = &mut specification.site.water.harbour {
        harbour.bearing_degrees = Bearing::Degrees(degrees);
    }
    specification
}

pub fn with_land_cover() -> Specification {
    let mut specification = valid();
    let land_cover = &mut specification.site.land_cover;
    land_cover.outside = vec![OutsideCover::Farmland, OutsideCover::Forest];
    land_cover.inside = vec![InsideCover::Gardens];
    land_cover.bands.push(Band {
        cover: OutsideCover::Forest,
        bearing_degrees: Bearing::Word(CompassRegion::North),
        depth_metres: length(300.0),
    });
    specification
}

pub fn weight(value: f64) -> Weight {
    Weight::new(value).expect("test input is a valid weight")
}

pub fn rule(rule: Rule, weight: Weight) -> Constraint {
    Constraint {
        rule,
        hard: false,
        weight,
    }
}

pub fn compass_region(region: Bearing, width_degrees: Option<u16>) -> Constraint {
    let region = Rule::CompassRegion {
        region,
        width_degrees,
    };
    rule(region, Weight::DEFAULT)
}

pub fn placement(rotation_degrees: f64) -> Placement {
    Placement {
        x: length(1200.0),
        y: length(1300.0),
        rotation_degrees: Rotation::new(rotation_degrees).expect("test input is a valid turn"),
        width_metres: length(20.0),
        depth_metres: length(40.0),
    }
}

pub fn guildhall(pinned: bool) -> Landmark {
    Landmark {
        identifier: "guildhall".to_string(),
        name: "The Guildhall".to_string(),
        function: LandmarkFunction::Civic,
        label: Some("guildhall".to_string()),
        importance: Importance::Major,
        footprint: Some(Footprint {
            class: Some(FootprintClass::Medium),
            width_metres: None,
            depth_metres: Some(length(40.0)),
        }),
        constraints: Vec::new(),
        resolved: Some(placement(90.0)),
        pinned,
        notes: String::new(),
    }
}

pub fn with_landmark(landmark: Landmark) -> Specification {
    let mut specification = valid();
    specification.landmarks.push(landmark);
    specification
}

pub fn market(pinned: bool) -> OpenSpace {
    OpenSpace {
        identifier: "market".to_string(),
        kind: OpenSpaceKind::MarketSquare,
        constraints: Vec::new(),
        resolved: Some(placement(0.0)),
        pinned,
    }
}

pub fn bridge(river: &str) -> Crossing {
    Crossing {
        identifier: "ash_bridge".to_string(),
        river: river.to_string(),
        kind: RiverCrossingKind::Bridge,
        constraints: Vec::new(),
        resolved: None,
        pinned: false,
    }
}

pub fn with_market_and_bridge(market: OpenSpace, bridge: Crossing) -> Specification {
    let mut specification = valid();
    specification.open_space.push(market);
    specification.crossings.push(bridge);
    specification
}

pub fn wall(identifier: &str, encloses: &[&str]) -> Wall {
    Wall {
        identifier: identifier.to_string(),
        kind: WallKind::Stone,
        encloses: encloses.iter().map(ToString::to_string).collect(),
        along_water: AlongWater::Wall,
        separate_ring: false,
        gap_metres: None,
    }
}

pub fn with_walls(walls: Vec<Wall>) -> Specification {
    let mut specification = valid();
    let docks = District {
        identifier: "docks".to_string(),
        ..specification.districts[0].clone()
    };
    specification.districts.push(docks);
    specification.defences.walls = walls;
    specification
}

pub fn with_defences(citadel: Option<Citadel>, gates: Vec<Gate>) -> Specification {
    let mut specification = with_walls(vec![wall("town_wall", &["old_town", "docks"])]);
    specification.defences.citadel = citadel;
    specification.defences.gates = gates;
    specification
}

pub fn citadel(wall: &str, bearing_degrees: Bearing) -> Citadel {
    Citadel {
        identifier: "keep".to_string(),
        bearing_degrees,
        wall: wall.to_string(),
    }
}

pub fn gate(identifier: &str, wall: &str, kind: GateKind) -> Gate {
    Gate {
        kind,
        identifier: identifier.to_string(),
        wall: wall.to_string(),
    }
}

pub fn road_gate(road: &str, end: Option<RoadEnd>) -> GateKind {
    GateKind::RoadGate {
        road: road.to_string(),
        end,
    }
}

pub fn near(target: &str) -> Constraint {
    let near = Rule::Near {
        target: target.to_string(),
        within_metres: length(150.0),
    };
    rule(near, Weight::DEFAULT)
}

pub fn add_district(specification: &mut Specification, identifier: &str, rules: Vec<Constraint>) {
    let district = District {
        identifier: identifier.to_string(),
        constraints: rules,
        ..specification.districts[0].clone()
    };
    specification.districts.push(district);
}

pub fn identifiers(names: &[&str]) -> Vec<String> {
    names.iter().map(ToString::to_string).collect()
}

pub fn with_gate_and_citadel() -> Specification {
    let gates = vec![gate("west_gate", "town_wall", road_gate("west_road", None))];
    with_defences(Some(citadel("town_wall", Bearing::Degrees(0))), gates)
}

pub fn landmark_near(identifier: &str, targets: &[&str]) -> Landmark {
    let mut landmark = guildhall(false);
    landmark.identifier = identifier.to_string();
    landmark.constraints = targets.iter().map(|target| near(target)).collect();
    landmark
}

pub fn any_edge() -> impl Strategy<Value = CompassRegion> {
    let edges: Vec<CompassRegion> = CompassRegion::ALL
        .into_iter()
        .filter(|region| *region != CompassRegion::Centre)
        .collect();
    prop::sample::select(edges)
}

pub fn any_two_edges() -> impl Strategy<Value = (CompassRegion, CompassRegion)> {
    (any_edge(), any_edge())
        .prop_filter("two different stretches", |(first, second)| first != second)
}

pub fn any_valid_specification() -> impl Strategy<Value = Specification> {
    (
        "[A-Za-z ]{1,80}",
        POPULATION,
        500.0..=5000.0_f64,
        any_two_edges(),
        any_two_edges(),
    )
        .prop_map(|(name, population, extent, river_ends, road_ends)| {
            let mut specification = valid();
            specification.identity.name = name;
            specification.settlement.population = population;
            specification.site.extent_metres = length(extent);
            specification.site.water.rivers[0].from = river_ends.0;
            specification.site.water.rivers[0].to = river_ends.1;
            specification.roads[0].ends =
                [RoadEnd::Region(road_ends.0), RoadEnd::Region(road_ends.1)];
            specification
        })
}

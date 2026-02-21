#[derive(Clone, Copy, Debug)]
pub enum RoutePart {
    Highway,
    Railway,
    None,
}

// North, East, South, West
#[derive(Debug)]
pub struct Network(RoutePart, RoutePart, RoutePart, RoutePart);

#[derive(Debug)]
pub struct Route {
    networks: [Option<Network>; 2],
}

#[derive(Clone, Copy, Debug)]
pub enum RouteKind {
    HighwayStraight,
    HighwayCurved,
    HighwayTjunktion,
    RailwayStraight,
    RailwayCurved,
    RailwayTjunktion,
    // Connections
    Overpass,
    StationStraight,
    StationCurved,
    // Special Routes
    HighwayCrossroad,
    RailwayCrossroad,
    HighwayStraightRailwayStraight,
    HighwayCurvedRailwayCurved,
    HighwayRailwayTjunktion,
    RailwayHighwayTjunktion,
}

pub const SPECIAL_ROUTES: [RouteKind; 6] = [
    RouteKind::HighwayCrossroad,
    RouteKind::RailwayCrossroad,
    RouteKind::HighwayStraightRailwayStraight,
    RouteKind::HighwayCurvedRailwayCurved,
    RouteKind::HighwayRailwayTjunktion,
    RouteKind::RailwayHighwayTjunktion,
];

impl RouteKind {
    pub fn get_route(&self) -> Route {
        match self {
            // highways
            Self::HighwayStraight => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::None,
                        RoutePart::Highway,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            Self::HighwayCurved => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::Highway,
                        RoutePart::None,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            Self::HighwayTjunktion => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::Highway,
                        RoutePart::Highway,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            // railways
            Self::RailwayStraight => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Railway,
                        RoutePart::None,
                        RoutePart::Railway,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            Self::RailwayCurved => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Railway,
                        RoutePart::Railway,
                        RoutePart::None,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            Self::RailwayTjunktion => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Railway,
                        RoutePart::Railway,
                        RoutePart::Railway,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            // Connections & Overpass
            Self::Overpass => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::None,
                        RoutePart::Highway,
                        RoutePart::None,
                    )),
                    Option::Some(Network(
                        RoutePart::None,
                        RoutePart::Railway,
                        RoutePart::None,
                        RoutePart::Railway,
                    )),
                ],
            },
            Self::StationStraight => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::None,
                        RoutePart::Railway,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            Self::StationCurved => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::Railway,
                        RoutePart::None,
                        RoutePart::None,
                    )),
                    Option::None,
                ],
            },
            // Special
            Self::HighwayCrossroad => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::Highway,
                        RoutePart::Highway,
                        RoutePart::Highway,
                    )),
                    Option::None,
                ],
            },
            Self::RailwayCrossroad => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Railway,
                        RoutePart::Railway,
                        RoutePart::Railway,
                        RoutePart::Railway,
                    )),
                    Option::None,
                ],
            },
            Self::HighwayStraightRailwayStraight => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::Railway,
                        RoutePart::Highway,
                        RoutePart::Railway,
                    )),
                    Option::None,
                ],
            },
            Self::HighwayCurvedRailwayCurved => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::Highway,
                        RoutePart::Railway,
                        RoutePart::Railway,
                    )),
                    Option::None,
                ],
            },
            Self::HighwayRailwayTjunktion => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Highway,
                        RoutePart::Railway,
                        RoutePart::Railway,
                        RoutePart::Railway,
                    )),
                    Option::None,
                ],
            },
            Self::RailwayHighwayTjunktion => Route {
                networks: [
                    Option::Some(Network(
                        RoutePart::Railway,
                        RoutePart::Highway,
                        RoutePart::Highway,
                        RoutePart::Highway,
                    )),
                    Option::None,
                ],
            },
        }
    }
}

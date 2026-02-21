use std::fmt;

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

impl RoutePart {
    fn is_none(&self) -> bool {
        if let RoutePart::None = self {
            return true;
        }

        false
    }

    fn is_some(&self) -> bool {
        !self.is_none()
    }

    fn to_char(&self) -> char {
        match self {
            RoutePart::Highway => 'H',
            RoutePart::Railway => 'R',
            RoutePart::None => ' ',
        }
    }
}

impl Route {
    pub fn get_image_as_string(&self) -> String {
        self.to_char_view().to_string()
    }

    fn to_char_view(&self) -> RouteCharView {
        let mut view = [[' '; 3]; 3];
        for network in &self.networks {
            if let Some(Network(north, east, south, west)) = network {
                if north.is_some() {
                    view[0][1] = north.to_char();
                }
                if east.is_some() {
                    view[1][2] = east.to_char();
                }
                if south.is_some() {
                    view[2][1] = south.to_char();
                }
                if west.is_some() {
                    view[1][0] = west.to_char();
                }

                if view[1][1] != ' ' {
                    // it means it's a second disconnected network
                    view[1][1] = 'O';
                } else {
                    view[1][1] = '*';
                }
            }
        }
        RouteCharView { view }
    }
}

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

struct RouteCharView {
    view: [[char; 3]; 3],
}

impl fmt::Display for RouteCharView {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.view {
            let row: String = row.iter().collect();
            writeln!(f, "{}", row)?;
        }
        Ok(())
    }
}

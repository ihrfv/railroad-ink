use super::route::RouteKind;
use rand::RngExt;

pub const DICE_REGULAR_ROUTES: Dice = Dice {
    routes: [
        RouteKind::HighwayStraight,
        RouteKind::HighwayCurved,
        RouteKind::HighwayTjunktion,
        RouteKind::RailwayStraight,
        RouteKind::RailwayCurved,
        RouteKind::RailwayTjunktion,
    ],
};

pub const DICE_CONNECTION_ROUTES: Dice = Dice {
    routes: [
        RouteKind::Overpass,
        RouteKind::StationStraight,
        RouteKind::StationCurved,
        RouteKind::Overpass,
        RouteKind::StationStraight,
        RouteKind::StationCurved,
    ],
};

pub struct Dice {
    routes: [RouteKind; 6],
}

impl Dice {
    pub fn roll(&self) -> RouteKind {
        let dice_roll = rand::rng().random_range(0..self.routes.len());
        self.routes[dice_roll]
    }
}

pub struct FullRegionPoint {
    i: f64,
    j: f64,
    n: f64,
}

pub struct PartialRegionPoint {
    n: f64,
}

pub enum RegionPoint {
    Full(FullRegionPoint),
    Partial(PartialRegionPoint),
}
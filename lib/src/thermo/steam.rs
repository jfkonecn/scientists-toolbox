
// https://github.com/jfkonecn/thermo/blob/feature/issue-42/thermo/steam_properties.py


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

static s: &'static [PartialRegionPoint] = &[
    PartialRegionPoint {
        n: 1.0,  
    }
];

pub fn function() {
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

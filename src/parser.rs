use nalgebra::Vector3;
use regex::Regex;

use crate::shapes::polygon::Polygon;
pub fn parse_ascii_stl(input: &str) -> Result<Vec<Polygon>, String> {
    let mut polygons = Vec::<Polygon>::new();
    let regex = Regex::new(
        r"facet normal  ([\w\+\-\.]+)  ([\w\+\-\.]+)  ([\w\+\-\.]+)\s*outer loop\s*vertex  ([\w\+\-\.]+)  ([\w\+\-\.]+)  ([\w\+\-\.]+)\s*vertex  ([\w\+\-\.]+)  ([\w\+\-\.]+)  ([\w\+\-\.]+)\s*vertex  ([\w\+\-\.]+)  ([\w\+\-\.]+)  ([\w\+\-\.]+)\s*endloop\s*endfacet",
    )
    .unwrap();

    for capture in regex.captures_iter(input) {
        polygons.push(Polygon {
            vertices: [
                Vector3::<f64>::new(
                    str::parse::<f64>(&capture[4]).unwrap(),
                    str::parse::<f64>(&capture[5]).unwrap(),
                    str::parse::<f64>(&capture[6]).unwrap(),
                ),
                Vector3::<f64>::new(
                    str::parse::<f64>(&capture[7]).unwrap(),
                    str::parse::<f64>(&capture[8]).unwrap(),
                    str::parse::<f64>(&capture[9]).unwrap(),
                ),
                Vector3::<f64>::new(
                    str::parse::<f64>(&capture[10]).unwrap(),
                    str::parse::<f64>(&capture[11]).unwrap(),
                    str::parse::<f64>(&capture[12]).unwrap(),
                ),
            ],
            normal: Vector3::<f64>::new(
                str::parse::<f64>(&capture[1]).unwrap(),
                str::parse::<f64>(&capture[2]).unwrap(),
                str::parse::<f64>(&capture[3]).unwrap(),
            ),
        })
    }

    Ok(polygons)
}

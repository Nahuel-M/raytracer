
use crate::shape::{mesh::Mesh, triangle::Triangle};

use regex::Regex;

#[allow(dead_code)]
pub fn parse_ascii_stl(input: &str) -> Result<Mesh, String> {
    let mut polygons = Vec::<Triangle>::new();
    let regex = Regex::new(
        r"facet normal\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*outer loop\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*endloop\s*endfacet",
    )
    .unwrap();

    for capture in regex.captures_iter(input) {
        polygons.push(Triangle::with_normal(
            (
                str::parse::<f64>(&capture[4]).unwrap(),
                str::parse::<f64>(&capture[5]).unwrap(),
                str::parse::<f64>(&capture[6]).unwrap(),
            ),
            (
                str::parse::<f64>(&capture[7]).unwrap(),
                str::parse::<f64>(&capture[8]).unwrap(),
                str::parse::<f64>(&capture[9]).unwrap(),
            ),
            (
                str::parse::<f64>(&capture[10]).unwrap(),
                str::parse::<f64>(&capture[11]).unwrap(),
                str::parse::<f64>(&capture[12]).unwrap(),
            ),
            (
                str::parse::<f64>(&capture[1]).unwrap(),
                str::parse::<f64>(&capture[2]).unwrap(),
                str::parse::<f64>(&capture[3]).unwrap(),
            ),
        ));
    }

    Ok(Mesh::new(polygons))
}


#[allow(dead_code)]
pub fn parse_ascii_stl_no_normals(input: &str) -> Result<Mesh, String> {
    let mut polygons = Vec::<Triangle>::new();
    let regex = Regex::new(
        r"facet normal\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*outer loop\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*endloop\s*endfacet",
    )
    .unwrap();

    for capture in regex.captures_iter(input) {
        polygons.push(Triangle::new( 
            (
                str::parse::<f64>(&capture[4]).unwrap(),
                str::parse::<f64>(&capture[5]).unwrap(),
                str::parse::<f64>(&capture[6]).unwrap(),
            ),
            (
                str::parse::<f64>(&capture[7]).unwrap(),
                str::parse::<f64>(&capture[8]).unwrap(),
                str::parse::<f64>(&capture[9]).unwrap(),
            ),
            (
                str::parse::<f64>(&capture[10]).unwrap(),
                str::parse::<f64>(&capture[11]).unwrap(),
                str::parse::<f64>(&capture[12]).unwrap(),
            ),
        ));
    }

    Ok(Mesh::new(polygons))
}
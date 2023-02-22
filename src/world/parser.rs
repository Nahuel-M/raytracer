// use std::sync::{Arc, RwLock};

use core::result::Result;

use crate::{algebra::vec3::Vec3, material::Material};

use super::{model::{Model, Vertex}, triangle::Triangle, World, VertexNormal};


// pub fn parse_ascii_stl<>(input: &str, world: &mut World, model_name: String) {
//     let regex = Regex::new(
//         r"facet normal\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*outer loop\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*vertex\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*([\w\+\-\.]+)\s*endloop\s*endfacet",
//     )
//     .unwrap();

//     let mut model = Model {
//         vertices: vec![],
//     };

//     for capture in regex.captures_iter(input) {
//         world.vertices.push(Vertex::new(
//             str::parse::<f64>(&capture[4]).unwrap(),
//             str::parse::<f64>(&capture[5]).unwrap(),
//             str::parse::<f64>(&capture[6]).unwrap(),
//         ));
//         world.vertices.push(Vertex::new(
//             str::parse::<f64>(&capture[7]).unwrap(),
//             str::parse::<f64>(&capture[8]).unwrap(),
//             str::parse::<f64>(&capture[9]).unwrap(),
//         ));
//         world.vertices.push(Vertex::new(
//             str::parse::<f64>(&capture[10]).unwrap(),
//             str::parse::<f64>(&capture[11]).unwrap(),
//             str::parse::<f64>(&capture[12]).unwrap(),
//         ));
//         let mut vertices : [Vertex; 3] = [Vertex::new(0.,0.,0.),Vertex::new(0.,0.,0.),Vertex::new(0.,0.,0.)];
//         vertices.clone_from_slice(&world.vertices[world.vertices.len()-3..]);
//         let triangle = Triangle {
//             vertices,
//             normal: Vec3::new(
//                 str::parse::<f64>(&capture[1]).unwrap(),
//                 str::parse::<f64>(&capture[2]).unwrap(),
//                 str::parse::<f64>(&capture[3]).unwrap(),
//             ),
//             material_index: 0,
//             vertex_normal: None,
//             vertex_color: None,
//         };
//         world.triangles.push(triangle);
//         model.vertices.extend_from_slice(&world.vertices[world.vertices.len()-3..]);
//     }
//     world.models.insert(model_name, model);
// }

// pub fn parse_ascii_obj(input: &str, world: &mut World, model_name: String) {
//     let mut model = Model {
//         vertices: vec![],
//     };

//     let starting_vertices_index = world.vertices.len();
//     let starting_vertex_normals_index = world.vertex_normals.len();

//     let vertex_regex = Regex::new(r"v\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)").unwrap();
//     for capture in vertex_regex.captures_iter(input) {
//         let vertex = Vertex::new(
//             str::parse::<f64>(&capture[1]).unwrap_or_else(|err| {
//                 println!("{}, {}", err, &capture[1]);
//                 1.
//             }),
//             str::parse::<f64>(&capture[2]).unwrap(),
//             str::parse::<f64>(&capture[3]).unwrap(),
//         );
//         world.vertices.push(vertex.clone());
//         model.vertices.push(vertex.clone());
//     }

//     let vertex_normal_regex =
//         Regex::new(r"vn\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)\s+([\w\+\-\.]+)").unwrap();
//     for capture in vertex_normal_regex.captures_iter(input) {
//         world.vertex_normals.push(VertexNormal(Vec3::new(
//             str::parse::<f64>(&capture[1]).unwrap(),
//             str::parse::<f64>(&capture[2]).unwrap(),
//             str::parse::<f64>(&capture[3]).unwrap(),
//         )));
//     }

//     let triangle_regex = Regex::new(r"f\s+([\w]+)\s+([\w]+)\s+([\w]+)").unwrap();
//     for capture in triangle_regex.captures_iter(input) {
//         let vertex_indexes = [
//             str::parse::<usize>(&capture[1]).unwrap() + starting_vertices_index - 1,
//             str::parse::<usize>(&capture[2]).unwrap() + starting_vertices_index - 1,
//             str::parse::<usize>(&capture[3]).unwrap() + starting_vertices_index - 1,
//         ];

//         let vertices : [Vertex; 3] = vertex_indexes
//             .iter()
//             .map(|&index| world.vertices[index].clone())
//             .collect::<Vec<Vertex>>()
//             .try_into()
//             .unwrap();

//         let vertex_normal_indexes = [
//             str::parse::<usize>(&capture[1]).unwrap() + starting_vertex_normals_index - 1,
//             str::parse::<usize>(&capture[2]).unwrap() + starting_vertex_normals_index - 1,
//             str::parse::<usize>(&capture[3]).unwrap() + starting_vertex_normals_index - 1,
//         ];

//         let mut normal = Vec3::cross(
//             &(vertices[1].get() - vertices[0].get()),
//             &(vertices[2].get() - vertices[0].get()),
//         ).normalize();
//         if normal.dot(&world.vertex_normals[vertex_normal_indexes[0]].0) < 0. {
//             normal *= -1.;
//         }

//         let triangle = Triangle {
//             vertices,
//             normal,
//             material_index: 0,
//             vertex_normal: Some(vertex_normal_indexes),
//             vertex_color: None,
//         };
//         world.triangles.push(triangle);
//     }
    
//     world.models.insert(model_name, model);
// }

pub fn parse_ascii_obj(input : &str, world : &mut World) -> Result<(), String>{
    let lines = input.lines();

    let (mut v_counter, mut vn_counter, mut vt_counter) = (0,0,0);
    let mut current_model = None;
    let mut current_smoothing = true;
    for (index, line) in lines.enumerate(){
        if line.is_empty(){ continue; }
        let (prefix, data) = line.split_once(' ').ok_or("Could not parse line ".to_string() + &index.to_string()[..])?;
        match prefix{
            "#" => {}, // Comment, ignore for parsing
            "o" | "g" => { // Currently treating groups as objects
                world.models.insert(data.to_string(), Model::default());
                current_model = world.models.get_mut(&data.to_string());
                current_smoothing = true;
            },
            "v" => { // Vertex
                v_counter += 1;
                match vec3_from_str(data){
                    Ok(vertex) => current_model.as_mut().unwrap().vertices.push(vertex),
                    Err(string) => return Err(string + "at line" + &index.to_string()[..]),
                }
            },
            "vn" => { // Vertex normal
                vn_counter += 1;
                match vec3_from_str(data){
                    Ok(vertex_normal) => current_model.as_mut().unwrap().vertex_normals.push(vertex_normal),
                    Err(string) => return Err(string + "at line" + &index.to_string()[..]),
                }
            },
            "vt" => { // Vertex UV
                vt_counter += 1;
                match uv_from_str(data){
                    Ok(uv) => current_model.as_mut().unwrap().vertex_uv.push(uv),
                    Err(string) => return Err(string + "at line" + &index.to_string()[..]),
                }
            },
            "usemtl" => { // Name of requested material
                current_model.as_mut().unwrap().material_name = data.to_string();
            },
            "s" => { // Face smoothing on / off
                match data{
                    "on" | "1" => current_smoothing = true,
                    "off" | "0" => current_smoothing = false,
                    _ => return Err("Invalid smoothing request at line ".to_string() + &index.to_string()[..])
                }
            },
            "f" => { // Face
                match face_from_str(data, current_smoothing){
                    Ok(mut faces) => {
                        let model = current_model.as_mut().unwrap();
                        faces.iter_mut()
                        .for_each(|face| {
                            fix_index_offsets_for_face(
                                face, 
                                v_counter - model.vertices.len() + 1,
                                vt_counter - model.vertex_uv.len() + 1,
                                vn_counter - model.vertex_normals.len() + 1);
                            calculate_normal_for_face(face, &model.vertices, &model.vertex_normals);
                        });
                        model.faces.append(&mut faces);
                    },
                    Err(string) => return Err(string + "at line" + &index.to_string()[..]),
                }
            },
            &_ => {}
        }
    }
    Ok(())
}

fn vec3_from_str(input : &str) -> Result<Vec3, String>{
    let mut numbers = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<f64>);
    Ok(Vertex::new(
        numbers.next().ok_or("Invalid number".to_string())?, 
        numbers.next().ok_or("Invalid number".to_string())?,
        numbers.next().ok_or("Invalid number".to_string())?)
    )
}

fn f64_from_str(input : &str) -> Result<f64, String>{
    str::parse::<f64>(input)
        .ok()
        .ok_or("Invalid number".to_string())
}

fn uv_from_str(input : &str) -> Result<(f64, f64), String>{
    let mut numbers = input
        .split_ascii_whitespace()
        .flat_map(str::parse::<f64>);
    Ok((numbers.next().ok_or("Invalid number".to_string())?, 
        numbers.next().ok_or("Invalid number".to_string())?)
    )
}

fn face_from_str(input : &str, smoothing: bool) -> Result<Vec<Triangle>, String>{
    let (vertex_indices, remainder) : (Vec<&str>, Vec<&str>) = input
        .split_ascii_whitespace()
        .map(|triplet| triplet
            .split_once('/')
            .unwrap_or((triplet, "")))
        .unzip();
    
    let (vertex_uv_indices, vertex_normal_indices) : (Vec<&str>, Vec<&str>) = remainder
        .iter()
        .map(|duplet| duplet
            .split_once('/')
            .unwrap_or((duplet, "")))
        .unzip();
    
    let vertex_indices = try_parse_usize(&vertex_indices);
    let vertex_uv_indices = try_parse_usize(&vertex_uv_indices);
    let vertex_normal_indices = try_parse_usize(&vertex_normal_indices);

    let vertex_triangles : Vec<[usize; 3]> = try_collect_triangles(&vertex_indices).into_iter().map(Option::unwrap).collect();
    let uv_triangles = try_collect_triangles(&vertex_uv_indices);
    let normal_triangles = try_collect_triangles(&vertex_normal_indices);

    let mut triangles = vec![];
    for ((vertices, vertex_uvs), vertex_normals) in vertex_triangles.into_iter().zip(uv_triangles).zip(normal_triangles){
        let triangle = Triangle{
            normal: Vec3::ZEROS,
            vertices,
            smoothing,
            vertex_normals,
            vertex_uvs,
        };
        triangles.push(triangle);
    }
    Ok(triangles)
}

fn try_parse_usize(vec: &[&str]) -> Vec<Option<usize>>{
    vec
        .iter()
        .map(|&index| str::parse::<usize>(index).ok())
        .collect()
}

fn try_collect_triangles(indices: &[Option<usize>]) -> Vec<Option<[usize; 3]>>{
    let vec = indices
        .iter()
        .flatten()
        .copied()
        .collect::<Vec<usize>>();

    if vec.is_empty(){
        return vec![];
    }

    let p1 = vec[0];
    vec
        .windows(2)
        .skip(1)
        .map(|window| Some([p1, window[0], window[1]]))
        .collect()

    // if vec.len() == 3{
    //     vec![vec.try_into().ok()]
    // } else if vec.len() == 4{
    //     vec![Some([vec[0], vec[1], vec[2]]), Some([vec[3], vec[2], vec[0]])]
    // } else {
    //     return vec![];
    // }
}

fn fix_index_offsets_for_face(face: &mut Triangle, object_lowest_v : usize, object_lowest_uv : usize, object_lowest_normal : usize){
    face.vertices[0] -= object_lowest_v;
    face.vertices[1] -= object_lowest_v;
    face.vertices[2] -= object_lowest_v;

    if let Some(uvs) = face.vertex_uvs.as_mut(){
        uvs[0] -= object_lowest_uv;
        uvs[1] -= object_lowest_uv;
        uvs[2] -= object_lowest_uv;
    }

    if let Some(normals) = face.vertex_normals.as_mut(){
        normals[0] -= object_lowest_normal;
        normals[1] -= object_lowest_normal;
        normals[2] -= object_lowest_normal;
    }
}

fn calculate_normal_for_face(face: &mut Triangle, vertices: &[Vertex], normals: &[VertexNormal]){
    let vertices : Vec<Vertex> = face.vertices.iter().map(|&index| vertices[index]).collect();
    let mut normal = Vec3::cross(
        &(vertices[1] - vertices[0]),
        &(vertices[2] - vertices[0]),
    ).normalize();
    if let Some(vertex_normals) = face.vertex_normals{
        if normal.dot(&normals[vertex_normals[0]]) < 0. {
            normal *= -1.;
        }
    }
    face.normal = normal;
}

pub(crate) fn parse_mtl(input: &str, world: &mut World) -> Result<(), String> {
    let lines = input.lines();
    let mut current_material = None;
    for (index, line) in lines.enumerate(){
        if line.is_empty(){ continue; }
        let (prefix, data) = line.split_once(' ').ok_or("Could not parse line ".to_string() + &index.to_string()[..])?;
        match prefix{
            "newmtl" => {
                world.materials.insert(data.to_string(), Material::default());
                current_material = world.materials.get_mut(&data.to_string());
            }, 
            "Ns" => {
                current_material.as_mut().unwrap().specular = f64_from_str(data)? / 1000.;
            },
            "Ka" => {

            },
            "Kd" => {
                current_material.as_mut().unwrap().diffuse_color = vec3_from_str(data)?;
            },
            "Ks" => {
                current_material.as_mut().unwrap().specular_color = vec3_from_str(data)?;
            },
            "Ke" => {
                current_material.as_mut().unwrap().luminance = vec3_from_str(data)?;
            },
            "Ni" => {
                current_material.as_mut().unwrap().ior = f64_from_str(data)?;
            },
            "d" => {
                current_material.as_mut().unwrap().refraction = 1. - f64_from_str(data)?;
            },
            _ => {}
        }
    }

    Ok(())
}
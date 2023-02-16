pub struct Shader{
    reflective_model : ReflectiveModel,
    refractive_model : RefractiveModel,
}

pub enum RefractiveModel{
    None,
    SchlickFresnell
}

pub struct ReflectiveModel{
    diffuse_model: DiffuseModel,
    specular_model: SpecularModel,
}

pub enum SpecularModel{
    None,
    CookTorrance(CookTorrance)
}

pub struct CookTorrance{
    distribution_function: SpecularDistributionFunction,
    geometry_function: SpecularGeometryFunction,

}

pub enum SpecularGeometryFunction{
    GGX,
}

pub enum SpecularDistributionFunction{
    GGX,
    Phong,

}

pub enum DiffuseModel{
    None,
    Lambertian
}
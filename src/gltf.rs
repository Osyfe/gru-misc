use gltf::*;
use crate::math::{Vec2, Vec3, Vec4};
use std::ops::Range;

#[derive(Debug)]
pub enum TextureOrConstant<const N: usize>
{
    Texture(String),
    Constant([f32; N])
}

#[derive(Debug)]
pub struct Mesh
{
    pub name: String,
    pub vertices: Range<usize>,
    pub indices: Range<usize>,
    pub diffuse_texture: TextureOrConstant<4>,
    pub normal_texture: Option<String>,
    pub roughness_texture: TextureOrConstant<1>
}

#[derive(Debug)]
pub struct Model
{
    pub positions: Vec<Vec3>,
    pub normals: Option<Vec<Vec3>>,
    pub tangents: Option<Vec<Vec3>>,
    pub tex_coords: Option<Vec<Vec2>>,
    pub indices: Vec<u32>,
    pub meshes: Vec<Mesh>
}

impl Model
{
    pub fn decode(gltf: &[u8], bin: &[u8]) -> Self
    {
        let Gltf { document: doc, blob: None } = Gltf::from_slice(gltf).unwrap() else { panic!("Binary glTF") };

        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut tangents = Vec::new();
        let mut tex_coords = Vec::new();
        let mut indices = Vec::new();

        let meshes = doc.meshes().map(|mesh|
        {
            let name = mesh.name().unwrap().to_owned();
            let mut primitives = mesh.primitives();
            let primitive = primitives.next().unwrap();
            if primitives.next().is_some() { panic!("Mesh {} has more than 1 primitive", name); }
            
            let vertex_start = positions.len() / 3;
            for attribute in primitive.attributes()
            {
                if let Some(vec) = match attribute.0
                {
                    Semantic::Positions => Some(&mut positions),
                    Semantic::Normals => Some(&mut normals),
                    Semantic::Tangents => Some(&mut tangents),
                    Semantic::TexCoords(_) => Some(&mut tex_coords),
                    _ => None
                } {
                    let accessor = attribute.1;
                    if accessor.data_type() != accessor::DataType::F32 { panic!("No F32 data"); }
                    let view = accessor.view().unwrap();
                    let data = &bin[view.offset()..];
                    let stride = view.stride().unwrap_or_else(|| accessor.size());
                    for i in 0..accessor.count()
                    {
                        let start = (stride * i) + accessor.offset();
                        let data = &data[start..(start + accessor.size())];
                        for float in data.array_chunks() { vec.push(f32::from_le_bytes(*float)); }
                    }
                }
            }
            let vertex_end = positions.len() / 3;
            assert!(vertex_end > vertex_start);

            //all meshes have to share the same data
            let num_vertices = positions.len() / 3;
            assert!(normals.len() == 0 || normals.len() / 3 == num_vertices);
            assert!(tangents.len() == 0 || tangents.len() / 4 == num_vertices);
            assert!(tex_coords.len() == 0 || tex_coords.len() / 2 == num_vertices);

            let index_start = indices.len();
            let accessor = primitive.indices().unwrap();
            let view = accessor.view().unwrap();
            let data = &bin[view.offset()..];
            let stride = view.stride().unwrap_or_else(|| accessor.size());
            for i in 0..accessor.count()
            {
                let start = (stride * i) + accessor.offset();
                let data = &data[start..(start + accessor.size())];
                match accessor.data_type()
                {
                    accessor::DataType::U16 => for int in data.chunks_exact(2) { indices.push(u16::from_le_bytes(int.try_into().unwrap()) as u32); },
                    accessor::DataType::U32 => for int in data.chunks_exact(4) { indices.push(u32::from_le_bytes(int.try_into().unwrap())); },
                    _ => unreachable!()
                }
            }
            let index_end = indices.len();
            
            let material = primitive.material();
            let diffuse_texture = match material.pbr_metallic_roughness().base_color_texture().map(|tex| tex.texture().source().source())
            {
                Some(image::Source::Uri { uri, .. }) => TextureOrConstant::Texture(uri.to_owned()),
                _ => TextureOrConstant::Constant(material.pbr_metallic_roughness().base_color_factor())
            };
            let normal_texture = match material.normal_texture().map(|tex| tex.texture().source().source())
            {
                Some(image::Source::Uri { uri, .. }) => Some(uri.to_owned()),
                _ => None
            };
            let roughness_texture = match material.pbr_metallic_roughness().metallic_roughness_texture().map(|tex| tex.texture().source().source())
            {
                Some(image::Source::Uri { uri, .. }) => TextureOrConstant::Texture(uri.to_owned()),
                _ => TextureOrConstant::Constant([material.pbr_metallic_roughness().roughness_factor()])
            };

            Mesh
            {
                name,
                vertices: vertex_start..vertex_end,
                indices: index_start..index_end,
                diffuse_texture,
                normal_texture,
                roughness_texture
            }
        }).collect();

        let positions = positions.array_chunks().cloned().map(Vec3::from).collect();
        let normals = if normals.len() != 0 { Some(normals.array_chunks().cloned().map(Vec3::from).collect()) } else { None };
        let tangents = if tangents.len() != 0 { Some(tangents.array_chunks().cloned().map(|floats| Vec4::from(floats).without_w()).collect()) } else { None };
        let tex_coords = if tex_coords.len() != 0 { Some(tex_coords.array_chunks().cloned().map(Vec2::from).collect()) } else { None };

        Self
        {
            positions,
            normals,
            tangents,
            tex_coords,
            indices,
            meshes
        }
    }
}

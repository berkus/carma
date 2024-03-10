//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{self, brender::read_c_string, Error},
    bevy::prelude::*,
    byteorder::{BigEndian, ReadBytesExt},
    fehler::{throw, throws},
    std::io::BufRead,
};

//------------------------------------------------------------------
/// Read resource from a stream.
pub trait FromStream {
    type Output;
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Result<Self::Output, Error>;
}

//------------------------------------------------------------------
/// Read an array resource with a specified count from a stream.
pub trait FromStreamExt {
    type Output;
    fn from_stream_ext<S: ReadBytesExt + BufRead>(
        source: &mut S,
        count: usize,
    ) -> Result<Self::Output, Error>;
}

//------------------------------------------------------------------
/// A binary resource file consisting of chunks with specific size.
/// Reading from such file yields array of chunk results, some of
/// these chunks are service, some are useful to the client.
#[derive(Default)]
struct ChunkHeader {
    chunk_type: u32,
    /// size of chunk without the header
    size: u32,
}

impl FromStream for ChunkHeader {
    type Output = ChunkHeader;
    #[throws(support::Error)]
    fn from_stream<R: ReadBytesExt>(source: &mut R) -> Self::Output {
        let chunk_type = source.read_u32::<BigEndian>()?;
        let size = source.read_u32::<BigEndian>()?;
        debug!("Loaded chunk type {} size {}", chunk_type, size);
        Self::Output { chunk_type, size }
    }
}

//------------------------------------------------------------------
/// A by-name reference.
pub struct NameRefChunk {
    pub identifier: String,
}

impl FromStream for NameRefChunk {
    type Output = NameRefChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let identifier = read_c_string(source)?;
        trace!("... {}", identifier);
        Self::Output { identifier }
    }
}

//------------------------------------------------------------------
/// Chunk types.
pub mod chunk {
    pub const END: u32 = 0x0;
    pub const PIXELMAP: u32 = 0x3;
    pub const MATERIAL: u32 = 0x4;
    pub const ANIM: u32 = 0xf;
    pub const ANIM_TRANSFORM: u32 = 0x10;
    pub const ANIM_RATE: u32 = 0x11;
    pub const FILE_INFO: u32 = 0x12;
    pub const PIVOT: u32 = 0x15;
    pub const MATERIAL_INDEX: u32 = 0x16;
    pub const VERTICES: u32 = 0x17;
    pub const VERTEX_UV: u32 = 0x18;
    pub const FACE_MATERIAL: u32 = 0x1a;
    pub const COLOUR_MAP_REF: u32 = 0x1c;
    pub const INDEX_BLEND_REF: u32 = 0x1e;
    pub const INDEX_SHADE_REF: u32 = 0x1f; // RENDERTAB_REF
    pub const SCREENDOOR_REF: u32 = 0x20;
    pub const PIXELS: u32 = 0x21;
    pub const ADD_MAP: u32 = 0x22; // Connect a map to an indexed pixelmap
    pub const ACTOR: u32 = 0x23;
    pub const ACTOR_MODEL: u32 = 0x24;
    pub const ACTOR_TRANSFORM: u32 = 0x25;
    pub const ACTOR_MATERIAL: u32 = 0x26;
    pub const ACTOR_LIGHT: u32 = 0x27;
    pub const ACTOR_CAMERA: u32 = 0x28;
    pub const ACTOR_BOUNDS: u32 = 0x29;
    pub const ACTOR_ADD_CHILD: u32 = 0x2a;
    pub const TRANSFORM_MATRIX34: u32 = 0x2b;
    pub const TRANSFORM_MATRIX34_LP: u32 = 0x2c;
    pub const TRANSFORM_QUAT: u32 = 0x2d;
    pub const TRANSFORM_EULER: u32 = 0x2e;
    pub const TRANSFORM_LOOK_UP: u32 = 0x2f;
    pub const TRANSFORM_TRANSLATION: u32 = 0x30;
    pub const TRANSFORM_IDENTITY: u32 = 0x31;
    pub const BOUNDS: u32 = 0x32;
    pub const LIGHT: u32 = 0x33;
    pub const CAMERA: u32 = 0x34;
    pub const FACES: u32 = 0x35;
    pub const MODEL: u32 = 0x36;
    pub const ACTOR_CLIP_PLANE: u32 = 0x37;
    pub const PLANE: u32 = 0x38;
}

//------------------------------------------------------------------
/// Types for FILE_INFO chunk.
pub mod file_type {
    pub const NONE: u32 = 0x0;
    pub const ACTOR: u32 = 0x1;
    pub const PIXELMAP: u32 = 0x2;
    pub const LIGHT: u32 = 0x3;
    pub const CAMERA: u32 = 0x4;
    pub const MATERIAL: u32 = 0x5;
    pub const MODEL: u32 = 0xface;
    pub const ANIM: u32 = 0x0a11;
    pub const TREE: u32 = 0x5eed;
}

//------------------------------------------------------------------
/// Types for ACTOR chunk.
pub mod actor_type {
    pub const NONE: u32 = 0x0;
    pub const MODEL: u32 = 0x1;
    pub const LIGHT: u32 = 0x2;
    pub const CAMERA: u32 = 0x3;
    pub const BOUNDS: u32 = 0x5;
    pub const BOUNDS_CORRECT: u32 = 0x6;
    pub const CLIP_PLANE: u32 = 0x7;
}

//------------------------------------------------------------------
/// Rendering style for ACTOR chunk.
pub mod actor_render_style {
    pub const DEFAULT: u32 = 0x0;
    pub const NONE: u32 = 0x1;
    pub const POINTS: u32 = 0x2;
    pub const EDGES: u32 = 0x3;
    pub const FACES: u32 = 0x4;
    pub const BOUNDING_POINTS: u32 = 0x5;
    pub const BOUNDING_EDGES: u32 = 0x6;
    pub const BOUNDING_FACES: u32 = 0x7;
}

//------------------------------------------------------------------
/// Order of rotations in a Euler transform.
pub mod euler_angle_order {
    pub const XYZ_S: u32 = 0x0;
    pub const XYX_S: u32 = 0x1;
    pub const XZY_S: u32 = 0x2;
    pub const XZX_S: u32 = 0x3;
    pub const YZX_S: u32 = 0x4;
    pub const YZY_S: u32 = 0x5;
    pub const YXZ_S: u32 = 0x6;
    pub const YXY_S: u32 = 0x7;
    pub const ZXY_S: u32 = 0x8;
    pub const ZXZ_S: u32 = 0x9;
    pub const ZYX_S: u32 = 0xa;
    pub const ZYZ_S: u32 = 0xb;
    pub const ZYX_R: u32 = 0xc;
    pub const XYX_R: u32 = 0xd;
    pub const YZX_R: u32 = 0xe;
    pub const XZX_R: u32 = 0xf;
    pub const XZY_R: u32 = 0x10;
    pub const YZY_R: u32 = 0x11;
    pub const ZXY_R: u32 = 0x12;
    pub const YXY_R: u32 = 0x13;
    pub const YXZ_R: u32 = 0x14;
    pub const ZXZ_R: u32 = 0x15;
    pub const XYZ_R: u32 = 0x16;
    pub const ZYZ_R: u32 = 0x17;
}

//------------------------------------------------------------------
/// Type for LIGHT chunk.
pub mod light_type {
    pub const POINT: u32 = 0x0;
    pub const DIRECT: u32 = 0x1;
    pub const SPOT: u32 = 0x2;
    pub const VIEW_POINT: u32 = 0x4;
    pub const VIEW_DIRECT: u32 = 0x5;
    pub const VIEW_SPOT: u32 = 0x6;
}

//------------------------------------------------------------------
/// Type for CAMERA chunk.
pub mod camera_type {
    pub const PARALLEL: u32 = 0x0;
    pub const PERSPECTIVE: u32 = 0x1;
}

//------------------------------------------------------------------
/// Type for payload of a PIXELMAP chunk.
pub mod pixelmap_type {
    pub const INDEX_1: u8 = 0x0;
    pub const INDEX_2: u8 = 0x1;
    pub const INDEX_4: u8 = 0x2;
    pub const INDEX_8: u8 = 0x3;
    pub const RGB_555: u8 = 0x4;
    pub const RGB_565: u8 = 0x5;
    pub const RGB_888: u8 = 0x6;
    pub const RGBX_888: u8 = 0x7;
    pub const RGBA_888: u8 = 0x8;
    pub const YUYV_8888: u8 = 0x9;
    pub const YUV_888: u8 = 0xa;
    pub const DEPTH_16: u8 = 0xb;
    pub const DEPTH_32: u8 = 0xc;
    pub const ALPHA_8: u8 = 0xd;
    pub const INDEXA_88: u8 = 0xe;
}

// =================
// Universal chunks:

//------------------------------------------------------------------
pub struct FileInfoChunk {
    pub file_type: u32,
    pub version: u32,
}

impl FromStream for FileInfoChunk {
    type Output = FileInfoChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let file_type = source.read_u32::<BigEndian>()?;
        let version = source.read_u32::<BigEndian>()?;
        Self::Output { file_type, version }
    }
}

// =================
// Model chunks: (BrModelLoadMany)

//------------------------------------------------------------------
pub struct ModelChunk {
    flags: u16,
    identifier: String,
}

impl FromStream for ModelChunk {
    type Output = ModelChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let flags = source.read_u16::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        Self::Output { flags, identifier }
    }
}

//------------------------------------------------------------------
pub struct MaterialIndexChunk {
    materials: Vec<String>,
}

impl FromStream for MaterialIndexChunk {
    type Output = MaterialIndexChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()?;
        let mut materials = Vec::<String>::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            let v = read_c_string(source)?;
            trace!("... ref ({})", v);
            materials.push(v);
        }
        Self::Output { materials }
    }
}

//------------------------------------------------------------------
struct Vec2f {
    x: f32,
    y: f32,
}

impl FromStream for Vec2f {
    type Output = Vec2f;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let x = source.read_f32::<BigEndian>()?;
        let y = source.read_f32::<BigEndian>()?;
        Self::Output { x, y }
    }
}

//------------------------------------------------------------------
type Vertex = Vec3f;

struct Vec3f {
    x: f32,
    y: f32,
    z: f32,
}

impl FromStream for Vec3f {
    type Output = Vec3f;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let x = source.read_f32::<BigEndian>()?;
        let y = source.read_f32::<BigEndian>()?;
        let z = source.read_f32::<BigEndian>()?;
        Self::Output { x, y, z }
    }
}

//------------------------------------------------------------------
struct Vec4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl FromStream for Vec4f {
    type Output = Vec4f;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let x = source.read_f32::<BigEndian>()?;
        let y = source.read_f32::<BigEndian>()?;
        let z = source.read_f32::<BigEndian>()?;
        let w = source.read_f32::<BigEndian>()?;
        Self::Output { x, y, z, w }
    }
}

//------------------------------------------------------------------
pub struct VertexUV {
    pub u: f32,
    pub v: f32,
}

impl FromStream for VertexUV {
    type Output = VertexUV;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let u = source.read_f32::<BigEndian>()?;
        let v = source.read_f32::<BigEndian>()?;
        Self::Output { u, v }
    }
}

//------------------------------------------------------------------
pub struct VerticesChunk {
    vertices: Vec<Vertex>,
}

impl FromStream for VerticesChunk {
    type Output = VerticesChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()?;
        let mut vertices = Vec::<Vertex>::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            let v = Vertex::from_stream(source)?; // IoVertex::..
            vertices.push(v); // v.0
        }
        Self::Output { vertices }
    }
}

//------------------------------------------------------------------
pub struct VertexUvChunk {
    pub uvs: Vec<VertexUV>,
}

impl FromStream for VertexUvChunk {
    type Output = VertexUvChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()?;
        let mut uvs = Vec::<VertexUV>::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            let v = VertexUV::from_stream(source)?;
            uvs.push(v);
        }
        Self::Output { uvs }
    }
}

//------------------------------------------------------------------
#[derive(Default)]
struct Face {
    v1: u16,
    v2: u16,
    v3: u16,
    smoothing: u16,
    flags: u8,
}

impl FromStream for Face {
    type Output = Face;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let v1 = source.read_u16::<BigEndian>()?;
        let v2 = source.read_u16::<BigEndian>()?;
        let v3 = source.read_u16::<BigEndian>()?;
        let smoothing = source.read_u16::<BigEndian>()?;
        let flags = source.read_u8()?;
        Self::Output {
            v1,
            v2,
            v3,
            smoothing,
            flags,
        }
    }
}

//------------------------------------------------------------------
pub struct FacesChunk {
    faces: Vec<Face>,
}

impl FromStream for FacesChunk {
    type Output = FacesChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()?;
        let mut faces = Vec::<Face>::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            let f = Face::from_stream(source)?;
            faces.push(f);
        }
        Self::Output { faces }
    }
}

//------------------------------------------------------------------
pub struct FaceMaterialChunk {
    pub face_material_indices: Vec<u16>,
}

impl FromStreamExt for FaceMaterialChunk {
    type Output = FaceMaterialChunk;
    #[throws(support::Error)]
    fn from_stream_ext<S: ReadBytesExt + BufRead>(source: &mut S, count: usize) -> Self::Output {
        let mut face_material_indexes = Vec::with_capacity(count);
        for _ in 0..count {
            let index = source.read_u16::<BigEndian>()?;
            face_material_indexes.push(index);
        }
        Self::Output {
            face_material_indices,
        }
    }
}

//------------------------------------------------------------------
pub struct PivotChunk {
    pub pivot: Vec3f,
}

impl FromStream for PivotChunk {
    type Output = PivotChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let pivot = Vertex::from_stream(source)?;
        Self::Output { pivot }
    }
}

//------------------------------------------------------------------
struct Colour {
    r: f32,
    g: f32,
    b: f32,
}

impl FromStream for Colour {
    type Output = Colour;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let r = source.read_f32::<BigEndian>()?;
        let g = source.read_f32::<BigEndian>()?;
        let b = source.read_f32::<BigEndian>()?;
        Self::Output { r, g, b }
    }
}

// =================
// Material chunks: (BrMaterialLoadMany)

//------------------------------------------------------------------
pub struct MaterialChunk {
    color: Colour,
    opacity: u8,
    ka: f32,
    kd: f32,
    ks: f32,
    power: f32,
    flags: u16,
    map_transform_x: Vec2f,
    map_transform_y: Vec2f,
    map_transform_z: Vec2f,
    index_base: u8,
    index_range: u8,
    identifier: String,
}

impl FromStream for MaterialChunk {
    type Output = MaterialChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let color = Colour::from_stream(source)?;
        let opacity = source.read_u8()?;
        let ka = source.read_f32::<BigEndian>()?;
        let kd = source.read_f32::<BigEndian>()?;
        let ks = source.read_f32::<BigEndian>()?;
        let power = source.read_f32::<BigEndian>()?;
        let flags = source.read_u16::<BigEndian>()?;
        let map_transform_x = Vec2f::from_stream(source)?;
        let map_transform_y = Vec2f::from_stream(source)?;
        let map_transform_z = Vec2f::from_stream(source)?;
        let index_base = source.read_u8()?;
        let index_range = source.read_u8()?;
        let identifier = read_c_string(source)?;
        trace!("... {}", identifier);
        Self::Output {
            color,
            opacity,
            ka,
            kd,
            ks,
            power,
            flags,
            map_transform_x,
            map_transform_y,
            map_transform_z,
            index_base,
            index_range,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub type ColorMapRefChunk = NameRefChunk;

//------------------------------------------------------------------
pub type IndexShadeRefChunk = NameRefChunk;

// =================
// PixelMap chunks: (BrPixelmapLoadMany)

//------------------------------------------------------------------
pub struct PixelMapChunk {
    pub r#type: u8, // pixelmap_type::
    pub row_bytes: u16,
    pub width: u16,
    pub height: u16,
    pub origin_x: u16,
    pub origin_y: u16,
    pub identifier: String,
}

impl FromStream for PixelMapChunk {
    type Output = PixelMapChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let r#type = source.read_u8()?;
        let row_bytes = source.read_u16::<BigEndian>()?;
        let width = source.read_u16::<BigEndian>()?;
        let height = source.read_u16::<BigEndian>()?;
        let origin_x = source.read_u16::<BigEndian>()?;
        let origin_y = source.read_u16::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        trace!("... {}", identifier);
        Self::Output {
            r#type,
            row_bytes,
            width,
            height,
            origin_x,
            origin_y,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub struct PixelsChunk {
    pub units: u32,
    pub unit_bytes: u32,
    pub data: Vec<u8>,
}

impl FromStream for PixelsChunk {
    type Output = PixelsChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let units = source.read_u32::<BigEndian>()?;
        let unit_bytes = source.read_u32::<BigEndian>()?;

        let payload_size = (units * unit_bytes) as usize;
        let mut data = vec![0u8; payload_size];
        source.read_exact(&mut data)?;

        Self::Output {
            units,
            unit_bytes,
            data,
        }
    }
}

// =================
// Actor chunks: (BrActorLoadMany)

//------------------------------------------------------------------
pub struct ActorChunk {
    r#type: u8,       // actor_type
    render_style: u8, // actor_render_style
    identifier: String,
}

impl FromStream for ActorChunk {
    type Output = ActorChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let r#type = source.read_u8()?;
        let render_style = source.read_u8()?;
        let identifier = read_c_string(source)?;
        Self::Output {
            r#type,
            render_style,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub type ActorModelChunk = NameRefChunk;

//------------------------------------------------------------------
pub type ActorMaterialChunk = NameRefChunk;

//------------------------------------------------------------------
pub struct ActorTransformActionChunk {} // empty, simply attach transform on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorLightActionChunk {} // empty, simply attach light on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorCameraActionChunk {} // empty, simply attach camera on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorBoundsActionChunk {} // empty, simply attach bounds on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorClipPlaneActionChunk {} // empty, simply attach clip plane on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorAddChildActionChunk {} // empty, simply attach actor on top of stack to the actor

//------------------------------------------------------------------
pub struct TransformMatrix34Chunk {
    m: Vec<Vec3f>, // 4-element vector of Vec3f
}

impl FromStream for TransformMatrix34Chunk {
    type Output = TransformMatrix34Chunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut m = Vec::with_capacity(4);
        for _ in 0..4 {
            m.push(Vec3f::from_stream(source)?);
        }
        Self::Output { m }
    }
}

//------------------------------------------------------------------
pub struct TransformQuatChunk {
    q_x: f32,
    q_y: f32,
    q_z: f32,
    q_w: f32,
    t: Vec3f,
}

impl FromStream for TransformQuatChunk {
    type Output = TransformQuatChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let q_x = source.read_f32::<BigEndian>()?;
        let q_y = source.read_f32::<BigEndian>()?;
        let q_z = source.read_f32::<BigEndian>()?;
        let q_w = source.read_f32::<BigEndian>()?;
        let t = Vec3f::from_stream(source)?;
        Self::Output {
            q_x,
            q_y,
            q_z,
            q_w,
            t,
        }
    }
}

//------------------------------------------------------------------
pub type Angle = f32;

//------------------------------------------------------------------
pub struct TransformEulerChunk {
    e_order: u8,
    e_a: Angle,
    e_b: Angle,
    e_c: Angle,
    t: Vec3f,
}

impl FromStream for TransformEulerChunk {
    type Output = TransformEulerChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let e_order = source.read_u8()?;
        let e_a = source.read_f32::<BigEndian>()?;
        let e_b = source.read_f32::<BigEndian>()?;
        let e_c = source.read_f32::<BigEndian>()?;
        let t = Vec3f::from_stream(source)?;
        Self::Output {
            e_order,
            e_a,
            e_b,
            e_c,
            t,
        }
    }
}

//------------------------------------------------------------------
pub struct TransformLookUpChunk {
    look: Vec3f,
    up: Vec3f,
    t: Vec3f,
}

impl FromStream for TransformLookUpChunk {
    type Output = TransformLookUpChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let look = Vec3f::from_stream(source)?;
        let up = Vec3f::from_stream(source)?;
        let t = Vec3f::from_stream(source)?;
        Self::Output { look, up, t }
    }
}

//------------------------------------------------------------------
pub struct TransformTranslationChunk {
    t: Vec3f,
}

impl FromStream for TransformTranslationChunk {
    type Output = TransformTranslationChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let t = Vec3f::from_stream(source)?;
        Self::Output { t }
    }
}

//------------------------------------------------------------------
pub struct BoundsChunk {
    min: Vec3f,
    max: Vec3f,
}

impl FromStream for BoundsChunk {
    type Output = BoundsChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let min = Vec3f::from_stream(source)?;
        let max = Vec3f::from_stream(source)?;
        Self::Output { min, max }
    }
}

//------------------------------------------------------------------
pub struct LightChunk {
    light_type: u8,
    color: Colour,
    attn_c: f32,
    attn_l: f32,
    attn_q: f32,
    cone_inner: Angle,
    cone_outer: Angle,
    identifier: String,
}

impl FromStream for LightChunk {
    type Output = LightChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let light_type = source.read_u8()?;
        let color = Colour::from_stream(source)?;
        let attn_c = source.read_f32::<BigEndian>()?;
        let attn_l = source.read_f32::<BigEndian>()?;
        let attn_q = source.read_f32::<BigEndian>()?;
        let cone_inner = source.read_f32::<BigEndian>()?;
        let cone_outer = source.read_f32::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        Self::Output {
            light_type,
            color,
            attn_c,
            attn_l,
            attn_q,
            cone_inner,
            cone_outer,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub struct CameraChunk {
    camera_type: u8,
    fov: Angle,
    hither_z: f32,
    yon_z: f32,
    aspect: f32,
    identifier: String,
}

impl FromStream for CameraChunk {
    type Output = CameraChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let camera_type = source.read_u8()?;
        let fov = source.read_f32::<BigEndian>()?;
        let hither_z = source.read_f32::<BigEndian>()?;
        let yon_z = source.read_f32::<BigEndian>()?;
        let aspect = source.read_f32::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        Self::Output {
            camera_type,
            fov,
            hither_z,
            yon_z,
            aspect,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub struct PlaneChunk {
    equation: Vec4f,
}

impl FromStream for PlaneChunk {
    type Output = PlaneChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let equation = Vec4f::from_stream(source)?;
        Self::Output { equation }
    }
}

//------------------------------------------------------------------
/// All chunk types (probably useless, use ModelLoadChunks, ActorLoadChunks etc)
pub enum Chunk {
    // =================
    // Universal chunks:
    End(),
    FileInfo(FileInfoChunk),

    // =================
    // Model chunks: (BrModelLoadMany)
    Model(ModelChunk),
    MaterialIndex(MaterialIndexChunk),
    Vertices(VerticesChunk),
    VertexUV(VertexUvChunk),
    Faces(FacesChunk),
    FaceMaterial(FaceMaterialChunk), // FACE_MATERIAL - external size
    Pivot(PivotChunk),

    // =================
    // Material chunks: (BrMaterialLoadMany)
    Material(MaterialChunk),
    ColorMapRef(ColorMapRefChunk),
    IndexBlendRef(), // INDEX_BLEND_REF
    IndexShadeRef(IndexShadeRefChunk),
    ScreendoorRef(), // SCREENDOOR_REF

    // =================
    // PixelMap chunks: (BrPixelmapLoadMany)
    PixelMap(PixelMapChunk),
    Pixels(PixelsChunk),
    AddMap(),

    // =================
    // Actor chunks: (BrActorLoadMany)
    Actor(ActorChunk),
    ActorModel(ActorModelChunk),
    ActorTransform(ActorTransformActionChunk),
    ActorMaterial(ActorMaterialChunk),
    ActorLight(ActorLightActionChunk),
    ActorCamera(ActorCameraActionChunk),
    ActorBounds(ActorBoundsActionChunk),
    ActorClipPlane(ActorClipPlaneActionChunk),
    ActorAddChild(ActorAddChildActionChunk),

    TransformMatrix34(TransformMatrix34Chunk),
    TransformMatrix34LP(TransformMatrix34Chunk),
    TransformQuat(TransformQuatChunk),
    TransformEuler(TransformEulerChunk),
    TransformLookUp(TransformLookUpChunk),
    TransformTranslation(TransformTranslationChunk),
    TransformIdentity(),

    Bounds(BoundsChunk),
    Light(LightChunk),
    Camera(CameraChunk),
    Plane(PlaneChunk),
}

impl FromStream for Chunk {
    type Output = Chunk;

    /// General chunk reader, no logic, just i/o.
    #[throws(support::Error)]
    fn from_stream<R: ReadBytesExt + BufRead>(source: &mut R) -> Chunk {
        let header = ChunkHeader::from_stream(source)?;
        match header.chunk_type {
            // =================
            // Universal chunks:
            chunk::END => Chunk::End(),
            chunk::FILE_INFO => {
                trace!("Reading file info...");
                assert_eq!(header.size, 8);
                Chunk::FileInfo(FileInfoChunk::from_stream(source)?)
            }

            // =================
            // Model chunks: (BrModelLoadMany)
            chunk::MODEL => {
                trace!("Reading model...");
                Chunk::Model(ModelChunk::from_stream(source)?)
            }
            chunk::MATERIAL_INDEX => {
                trace!("Reading material index...");
                Chunk::MaterialIndex(MaterialIndexChunk::from_stream(source)?)
            }
            chunk::VERTICES => {
                trace!("Reading vertex list...");
                Chunk::Vertices(VerticesChunk::from_stream(source)?)
            }
            chunk::VERTEX_UV => {
                trace!("Reading uvmap list...");
                Chunk::VertexUV(VertexUvChunk::from_stream(source)?)
            }
            chunk::FACES => {
                trace!("Reading face list...");
                Chunk::Faces(FacesChunk::from_stream(source)?)
            }
            chunk::FACE_MATERIAL => {
                trace!("Reading face material list...");
                Chunk::FaceMaterial(FaceMaterialChunk::from_stream_ext(
                    source,
                    (header.size / 2) as usize,
                )?)
            }
            chunk::PIVOT => {
                trace!("Reading pivot...");
                Chunk::Pivot(PivotChunk::from_stream(source)?)
            }

            // =================
            // Material chunks: (BrMaterialLoadMany)
            chunk::MATERIAL => {
                trace!("Reading material descriptor...");
                Chunk::Material(MaterialChunk::from_stream(source)?)
            }
            chunk::COLOUR_MAP_REF => {
                trace!("Reading pixelmap ref...");
                Chunk::ColorMapRef(ColorMapRefChunk::from_stream(source)?)
            }
            // ❌ IndexBlendRef(),                      // INDEX_BLEND_REF
            chunk::INDEX_SHADE_REF => {
                trace!("Reading rendertab (shade) ref...");
                Chunk::IndexShadeRef(IndexShadeRefChunk::from_stream(source)?)
            }
            // ❌ ScreendoorRef(),                      // SCREENDOOR_REF

            // =================
            // PixelMap chunks: (BrPixelmapLoadMany)
            chunk::PIXELMAP => {
                trace!("Reading pixelmap header...");
                Chunk::PixelMap(PixelMapChunk::from_stream(source)?)
            }
            chunk::PIXELS => {
                trace!("Reading pixelmap data...");
                Chunk::Pixels(PixelsChunk::from_stream(source)?)
            }
            // ❌ AddMap(),

            // =================
            // Actor chunks: (BrActorLoadMany)
            chunk::ACTOR => {
                trace!("Reading actor...");
                Chunk::Actor(ActorChunk::from_stream(source)?)
            }
            chunk::ACTOR_MODEL => {
                trace!("Reading actor model ref...");
                Chunk::ActorModel(ActorModelChunk::from_stream(source)?)
            }
            chunk::ACTOR_TRANSFORM => {
                trace!("Attaching actor transform...");
                Chunk::ActorTransform(ActorTransformActionChunk {})
                // @todo We should just pop transform and attach it to the actor on stack -- do it in actor reader tho
            }
            chunk::ACTOR_MATERIAL => {
                trace!("Reading actor material ref...");
                Chunk::ActorMaterial(ActorMaterialChunk::from_stream(source)?)
            }
            chunk::ACTOR_LIGHT => {
                trace!("Attaching actor light...");
                Chunk::ActorLight(ActorLightActionChunk {})
                // @todo We should just pop light and attach it to the actor on stack -- do it in actor reader tho
            }
            chunk::ACTOR_CAMERA => {
                trace!("Attaching actor camera...");
                Chunk::ActorCamera(ActorCameraActionChunk {})
                // @todo We should just pop camera and attach it to the actor on stack -- do it in actor reader tho
            }
            chunk::ACTOR_BOUNDS => {
                trace!("Attaching actor bounds...");
                Chunk::ActorBounds(ActorBoundsActionChunk {})
                // @todo We should just pop bounds and attach it to the actor on stack -- do it in actor reader tho
            }
            chunk::ACTOR_CLIP_PLANE => {
                trace!("Attaching actor clip plane...");
                Chunk::ActorClipPlane(ActorClipPlaneActionChunk {})
                // @todo We should just pop clip plane and attach it to the actor on stack -- do it in actor reader tho
            }
            chunk::ACTOR_ADD_CHILD => {
                trace!("Attaching sub-actor to actor...");
                Chunk::ActorAddChild(ActorAddChildActionChunk {})
                // @todo We should just pop actor and attach it to the actor on stack -- do it in actor reader tho
            }

            chunk::TRANSFORM_MATRIX34 => {
                trace!("Reading transform 3x4...");
                Chunk::TransformMatrix34(TransformMatrix34Chunk::from_stream(source)?)
            }
            chunk::TRANSFORM_MATRIX34_LP => {
                trace!("Reading transform 3x4 LP...");
                Chunk::TransformMatrix34LP(TransformMatrix34Chunk::from_stream(source)?)
            }
            chunk::TRANSFORM_QUAT => {
                trace!("Reading transform quat...");
                Chunk::TransformQuat(TransformQuatChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_EULER => {
                trace!("Reading transform Euler...");
                Chunk::TransformEuler(TransformEulerChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_LOOK_UP => {
                trace!("Reading transform look up...");
                Chunk::TransformLookUp(TransformLookUpChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_TRANSLATION => {
                trace!("Reading transform translation...");
                Chunk::TransformTranslation(TransformTranslationChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_IDENTITY => {
                trace!("Reading transform 3x4 LP...");
                Chunk::TransformIdentity()
            }

            chunk::BOUNDS => {
                trace!("Reading bounds...");
                Chunk::Bounds(BoundsChunk::from_stream(source)?)
            }
            chunk::LIGHT => {
                trace!("Reading light...");
                Chunk::Light(LightChunk::from_stream(source)?)
            }
            chunk::CAMERA => {
                trace!("Reading camera...");
                Chunk::Camera(CameraChunk::from_stream(source)?)
            }
            chunk::PLANE => {
                trace!("Reading plane...");
                Chunk::Plane(PlaneChunk::from_stream(source)?)
            }

            _ => unimplemented!(),
        }
    }
}

/*
 * Resource stack values.
 */
trait ResourceTag {
    fn as_any(&self) -> &dyn Any;
}

pub enum ResourceTag {
    Mark(), // Mark compound structure start
    ImagePlane(),
    PixelMap(Box<PixelMapChunk>),
    Material(Box<MaterialChunk>),
    Actor(Box<ActorChunk>),
    MaterialIndex(Box<MaterialIndexChunk>),
    Vertices(Box<VerticesChunk>),
    Faces(Box<FacesChunk>),
    Model(Box<ModelChunk>),
    Anim,
    AnimName,
    AnimTransform,
    AnimCount,
    AnimRate,
    FileInfo(Box<FileInfoChunk>),
    Pivot(Box<PivotChunk>),
    Transform(),
    Light(Box<LightChunk>),
    Camera(Box<CameraChunk>),
    Bounds(Box<BoundsChunk>),
    Plane(Box<PlaneChunk>),
}

pub mod stack {
    pub const MARK: u32 = 0;
    pub const IMAGE_PLANE: u32 = 1;
    pub const PIXELMAP: u32 = 2;
    pub const MATERIAL: u32 = 3;
    pub const ACTOR: u32 = 4;
    pub const MATERIAL_INDEX: u32 = 5;
    pub const VERTICES: u32 = 6;
    pub const FACES: u32 = 7;
    pub const MODEL: u32 = 8;
    pub const ANIM: u32 = 9;
    pub const ANIM_NAME: u32 = 10;
    pub const ANIM_TRANSFORM: u32 = 11;
    pub const ANIM_COUNT: u32 = 12;
    pub const ANIM_RATE: u32 = 13;
    pub const FILE_INFO: u32 = 14;
    pub const PIVOT: u32 = 15;
    pub const TRANSFORM: u32 = 16;
    pub const LIGHT: u32 = 17;
    pub const CAMERA: u32 = 18;
    pub const BOUNDS: u32 = 19;
    pub const PLANE: u32 = 20;
}

//------------------------------------------------------------------
/// Loading stack for resource chunks.
/// The per-resource loaders use it to consutrct
#[derive(Default)]
pub struct ResourceStack {
    stack: Vec<Box<dyn Any>>,
}

impl ResourceStack {
    pub fn push(&mut self, resource: ResourceTag) {
        self.stack.push(resource);
    }

    #[throws]
    pub fn pop<T>(&mut self) -> ResourceTag {
        if let Some(resource) = self.stack.pop()?.downcast_ref::<T>() {
            return resource;
        }

        throw!(support::Error::InvalidResourceType {
            expected: T::Tag,
            received: tag,
        });
    }

    /// Give mutable access to the stack top.
    pub fn top<T>(&mut self) -> Option<&mut ResourceTag> {
        if let Some(resource) = self.stack.last()?.downcast_ref::<T>() {
            return Some(resource);
        }
        None
    }
}

//------------------------------------------------------------------
// Tests.
//------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use {super::*, std::io::Cursor};

    #[test]
    fn test_load_face() {
        let mut data = Cursor::new(vec![0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0]);
        let f = Face::from_stream(&mut data).unwrap();
        assert_eq!(0xdead, f.v1);
        assert_eq!(0xbeef, f.v2);
        assert_eq!(0xcafe, f.v3);
        assert_eq!(0xbabe, f.flags);
    }
}

// use std::any::Any;

// trait Component {
//     // fn as_any(&self) -> &(dyn Any + '_) /*where Self: Sized*/ {
//     //     &self
//     // }
//     fn as_any(&self) -> &dyn Any;
// }

// struct A;
// struct B;

// impl A {
//     fn do_first_component_thing(&self) {
//         println!("First component thing");
//     }
// }
// impl B {
//     fn do_second_component_thing(&self) {
//         println!("Second component thing");
//     }
// }

// impl Component for A {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

// }

// impl Component for B {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }

// }

// fn main() {
//     let mut components: Vec<Box<dyn Component>> = Vec::new();
//     components.push(Box::new(A {}));
//     components.push(Box::new(B {}));

//     if let Some(component) =
//             components[0].as_any().downcast_ref::<A>() {
//         component.do_first_component_thing();
//     }

//     if let Some(component) =
//             components[1].as_any().downcast_ref::<B>() {
//         component.do_second_component_thing();
//     }
// }

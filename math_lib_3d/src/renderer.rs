#![allow(dead_code)]

use crate::euler_angles::*;
use crate::matrix4x3::Matrix4x3;
use crate::renderer::BackfaceMode::BackfaceModeCCW;
use crate::renderer::DestBlendMode::DestBlendModeInvSrcAlpha;
use crate::renderer::SourceBlendMode::*;
use crate::vector3::*;
use once_cell::sync::Lazy;
use std::sync::Mutex;

/////////////////////////////////////////////////////////////////////////////
//
// Simple constants, enums, macros
//
/////////////////////////////////////////////////////////////////////////////

// Maximum number of chars in a texture name (including the '\0')

pub const MAX_TEXTURE_NAME_CHARS: i32 = 64;

// Source blend functions

pub enum SourceBlendMode {
    SourceBlendModeSrcAlpha, // Normal source blending (default)
    SourceBlendModeOne,      // Use source color unmodified
    SourceBlendModeZero,     // No source contribution
}

// Destination blend functions

pub enum DestBlendMode {
    DestBlendModeInvSrcAlpha,
    // Inverse source alpha blend (default)
    DestBlendModeOne,
    // Additive blending
    DestBlendModeZero,
    // Discard current frame buffer pixel, blend with "black"
    DestBlendModeSrcColor, // Multiplicative blend (often used for lightmapping)
}

// Culling modes

pub enum BackfaceMode {
    BackfaceModeCCW,     // cull faces with counterclockwise screen-space order (default)
    BackfaceModeCW,      // cull faces with clockwise screen-space order
    BackfaceModeDisable, // Render all faces, regardless of screenspace vertex order
}

// Bitfield of options to the clear() function.

const CLEAR_FRAME_BUFFER: i32 = 1; // clear the frame buffer
const CLEAR_DEPTH_BUFFER: i32 = 2; // clear the zbuffer
const CLEAR_TO_CONSTANT_COLOR: i32 = 4; // clear frame buffer to constant color.  (By default, we clear to black)
const CLEAR_TO_FOG_COLOR: i32 = 8; // clear frame buffer to fog color.  (By default, we clear to black)

// Bitfield of vertex outcodes.  See the computeOutCode() function

const OUT_CODE_LEFT: i32 = 0x01;
const OUT_CODE_RIGHT: i32 = 0x02;
const OUT_CODE_BOTTOM: i32 = 0x04;
const OUT_CODE_TOP: i32 = 0x08;
const OUT_CODE_NEAR: i32 = 0x10;
const OUT_CODE_FAR: i32 = 0x20;
const OUT_CODE_FOG: i32 = 0x40;
const OUT_CODE_FRUSTUM_MASK: i32 = 0x3f; // bits to test if outside the frustom - don't worry about fog
const OUT_CODE_OFF_SCREEN_MASK: i32 = 0x1f; // bits to test if the projected point is off screen - far or fog don't matter

// Symbolic refresh rates that can be used when setting the video mode

const REFRESH_RATE_DEFAULT: i32 = -1;
const REFRESH_RATE_FASTEST: i32 = -2;

// Special texture handle that is always reserved for the "white texture,"
// whiet is a texture that is solid white.  This important texture is useful
// in a wide variety of circumstances

const WHITE_TEXTURE: i32 = 1;

// Macro to construct a color in 3D-form.
//
// This color value is NOT corrected for different pixel formats
// on different hardware formats.  It is for interface purposes
// only, NOT for frame buffer access.  The values are packed as follows:
//
// bits  0.. 7: blue
// bits  8..15: green
// bits 16..23: red
// bits 24..31: alpha
//
// It is also legal to pass a color like this: 0xAARRGGBB
//
// WARNING:
//
// The above rules apply to accessing a color via an integer
// value only, and have NOTHING to do with accessing the bytes
// in memory.

pub fn make_rgb(r: u32, g: u32, b: u32) -> u32 {
    //((unsigned int)(((unsigned int)(r) << 16) | ((unsigned int)(g) << 8) | ((unsigned int)(b))))
    r << 16 | g << 8 | b
}

pub fn make_argb(a: u32, r: u32, g: u32, b: u32) -> u32 {
    //((unsigned int)(((unsigned int)(a) << 24) |((unsigned int)(r) << 16) | ((unsigned int)(g) << 8) | ((unsigned int)(b))))
    a << 24 | r << 16 | g << 8 | b
}

pub fn get_a(argb: u32) -> u32 {
    //((int)(((unsigned int)(argb) >> 24U) & (unsigned int)0xff))
    argb >> 24u32 & 0xFF
}

pub fn get_r(argb: u32) -> u32 {
    //((int)(((unsigned int)(argb) >> 16U) & (unsigned int)0xff))
    argb >> 16u32 & 0xFF
}

pub fn get_g(argb: u32) -> u32 {
    //((int)(((unsigned int)(argb) >> 8U) & (unsigned int)0xff))
    argb >> 8u32 & 0xFF
}

pub fn get_b(argb: u32) -> u32 {
    //((int)((unsigned int)(argb) & (unsigned int)0xff))
    argb & 0xFF
}

/////////////////////////////////////////////////////////////////////////////
//
// Utility structures and classes
//
/////////////////////////////////////////////////////////////////////////////

//---------------------------------------------------------------------------
// struct VideoMode
//
// Defines a video mode - the resolution, color bit depth, and refresh rate.
// This struct is used when querying for a list of available modes, and
// also when setting the video mode.

pub struct VideoMode {
    x_res: i32,          // horizontal resolution, in pixels
    y_res: i32,          // vertical resolution, in pixels
    bits_per_pixel: i32, // currently only 16, 24, or 32 supported
    refresh_hz: i32,     // you can use one of kRefreshRateXxx constants when setting the video mode
}

//---------------------------------------------------------------------------
// struct RenderVertex - Untransformed, unlit vertex
// struct RenderVertexL - Untransformed, lit vertex
// struct RenderVertexTL - Transformed and lit vertex
//
// These structures are used to pass vertex data to the renderer.  Depending
// on what you want the renderer to do for you, you use a different
// structure.  For example, if the vertices are in modeling space and
// need to be transformed and projected to screen space, then you would
// use an untransformed vertex type.  If you want the renderer to apply
// lighting calculations, then you would use an unlit vertex type.  If you
// want to specify the vertex color manually, then use a pre-lit vertex type.
//
// See Section 15.7.2 for more information.

pub struct RenderVertex {
    pub p: Vector3, // position
    pub n: Vector3, // normal
    pub u: f32,     // texture mapping coordinate
    pub v: f32,     // texture mapping coordinate
}

pub struct RenderVertexL {
    pub p: Vector3, // position
    pub argb: u32,  // prelit diffuse color
    pub u: f32,     // texture mapping coordinate
    pub v: f32,     // texture mapping coordinate
}

// Transformed and lit vertex

pub struct RenderVertexTL {
    pub p: Vector3, // screen space position and z value
    pub oow: f32, // One Over W.  This is used for perspective projection.  Usually, you can just use 1/z.
    pub argb: u32, // prelit diffuse color (8 bits per component - 0xAARRGGBB)
    pub u: f32,   // texture mapping coordinate
    pub v: f32,   // texture mapping coordinate
}

//---------------------------------------------------------------------------
// struct RenderTri
//
// A single triangle for rendering.  It's just three indices.
// Notice that the indices are unsigned shorts.  This is for two reasons.
// First, using 16-bit rather than 32-bit indices effectively doubles the
// memory throughput for the index data.  Second, some graphics cards do
// not natively support 32-bit index data.
//
// This does put a limit on the max number of vertices in a single mesh
// at 65536.  This is usually too big of not a problem, since most large
// objects can easily be broken down into multiple meshes - in fact,
// you probably want to divide things up for visibility, etc, anyway.

pub struct RenderTri {
    // Todo: may not apply any more
    //unsigned short index[3];
    a: u16,
    b: u16,
    c: u16,
}

//---------------------------------------------------------------------------
// struct TextureReference
//
// Handy class for keeping track of a texture's name and handle.

pub struct TextureReference {
    // Name of the texture.  Usually this is a filename
    name: String, // [MAX_TEXTURE_NAME_CHARS]; // todo: revisit

    // Texture handle, within the graphics system
    handle: i32, // Todo: needed?
}

/////////////////////////////////////////////////////////////////////////////
//
// class Renderer
//
// Low-level renderer abstraction layer.
//
// See the .cpp file for more comments and opinions.
//
/////////////////////////////////////////////////////////////////////////////

// Instance stack system.  This is an OpenGL-like system to manage the
// current reference frame.  For example, by default, without instancing,
// all 3D coordinates submitted to the rendering system are assumed to be
// in world space.  Now, let's say we "instance" into an object's local
// reference frame, by specifying the position and orientation of the
// object.  Now any 3D coordinates we submit will be transformed from
// local space to world space and then into camera space.  Instancing
// can be performed multiple times, for example, to render a tire within a
// car.

pub struct InstanceInfo {
    // The model->world matrix
    model_to_world_matrix: Matrix4x3,
}

const MAX_INSTANCE_DEPTH: i32 = 8;

static mut INSTANCE_STACK_PTR: i32 = 0;

static INSTANCE_STACK: Lazy<Mutex<Vec<InstanceInfo>>> =
    Lazy::new(|| -> Mutex<Vec<InstanceInfo>> {
        let v: Vec<InstanceInfo> = Vec::new();
        Mutex::from(v)
    });

//[InstanceInfo, kMaxInstanceDepth];
pub struct GlobalFlag {
    need_to_compute_model_to_clip_matrix: bool,
}

impl GlobalFlag {
    pub fn new() -> GlobalFlag {
        GlobalFlag {
            need_to_compute_model_to_clip_matrix: true,
        }
    }
}

static NEED_TO_COMPUTE_MODEL_TO_CLIP_MATRIX: Lazy<Mutex<GlobalFlag>> =
    Lazy::new(|| -> Mutex<GlobalFlag> { Mutex::new(GlobalFlag::new()) });

pub struct Renderer {
    // Full screen resolution
    screen_x: i32,
    screen_y: i32,

    // Camera specification
    camera_pos: Vector3,
    camera_orient: EulerAngles,
    zoom_x: f32,
    zoom_y: f32,

    // Near/far clipping planes
    near_clip_plane: f32,
    far_clip_plane: f32,

    // The 2D output window
    window_x1: i32,
    window_y1: i32,
    window_x2: i32,
    window_y2: i32,
    window_size_x: i32,
    window_size_y: i32,

    // Zbuffer mode
    depth_buffer_read: bool,
    depth_buffer_write: bool,

    // Alpha blending
    blend_enable: bool,
    source_blend_mode: SourceBlendMode,
    dest_blend_mode: DestBlendMode,

    // Global constant color and opacity.
    constant_argb: u32,
    constant_opacity: f32,

    // Fog
    fog_enable: bool,
    fog_color: u32,
    fog_near: f32,
    fog_far: f32,

    // Lighting context.
    light_enable: bool,
    ambient_light_color: u32,
    directional_light_vector: Vector3,
    directional_light_color: u32,

    // Culling
    backface_mode: BackfaceMode,

    // Currently selected texture
    current_texture_handle: i32,

    // Texture clamp
    texture_clamp: bool,

    // Current world->camera matrix.  This will always be a rigid body
    // transform - it does not contain zoom or aspect ratio correction.
    world_to_camera_matrix: Matrix4x3,
}

impl Renderer {
    fn default() -> Self {
        // Slam some internal variables
        let mut renderer = Renderer {
            screen_x: 0,
            screen_y: 0,
            camera_pos: Vector3::zero(),
            camera_orient: EulerAngles::identity(),
            zoom_x: 1.0, // 90 degree field of view
            zoom_y: 0.0, // auto-compute
            near_clip_plane: 1.0,
            far_clip_plane: 1000.0,
            window_x1: 0,
            window_y1: 0,
            window_x2: 0,
            window_y2: 0,
            window_size_x: 0,
            window_size_y: 0,
            depth_buffer_read: true,
            depth_buffer_write: true,
            blend_enable: true,
            source_blend_mode: SourceBlendModeSrcAlpha,
            dest_blend_mode: DestBlendModeInvSrcAlpha,
            constant_argb: make_argb(255, 0, 0, 0),
            constant_opacity: 1.0,
            fog_enable: false,
            fog_color: make_rgb(255, 255, 255),
            fog_near: 0.0,
            fog_far: 1000.0,
            light_enable: true,
            ambient_light_color: make_rgb(64, 64, 64),
            directional_light_vector: Vector3 {
                x: 707.0,
                y: -0.707,
                z: 0.0,
            },
            directional_light_color: make_rgb(192, 192, 192),
            backface_mode: BackfaceModeCCW,
            current_texture_handle: 0,
            texture_clamp: false,
            world_to_camera_matrix: Matrix4x3::identity(),
        };
        // And now set the camera, to force some stuff to be recomputed
        renderer.set_camera(Vector3::zero(), EulerAngles::identity());

        // Set level 0 instance (the world) reference frame
        INSTANCE_STACK.lock().expect("vec").push(InstanceInfo {
            model_to_world_matrix: Matrix4x3::identity(),
        });

        renderer
    }

    // Screen resolution
    pub fn get_screen_x(&self) -> i32 {
        self.screen_x
    }
    pub fn get_screen_y(&self) -> i32 {
        self.screen_y
    }

    // Near/far clipping planes
    pub fn get_near_clipping_plane(&self) -> f32 {
        self.near_clip_plane
    }
    pub fn get_far_clipping_plane(&self) -> f32 {
        self.far_clip_plane
    }

    pub fn get_light_enable(&self) -> bool {
        self.light_enable
    }

    pub fn get_backface_mode(&self) -> &BackfaceMode {
        &self.backface_mode
    }

    pub fn get_current_texture(&self) -> i32 {
        self.current_texture_handle
    }

    pub fn get_world_to_camera_matrix(&self) -> &Matrix4x3 {
        &self.world_to_camera_matrix
    }

    pub fn set_camera(&mut self, pos: Vector3, orient: EulerAngles) {
        // Remember position and orientation

        self.camera_pos = pos;
        self.camera_orient = orient;

        // Recompute world -> camera matrix

        self.world_to_camera_matrix
            .setup_parent_to_local_euler_angles(&self.camera_pos, &self.camera_orient);

        // Upload this to the rendering API, if we have been initted

        // DirectX stuff
        // if (pD3DDevice != NULL) {
        //
        // // Convert our 4x3 matrix to D3D-style 4x4 matrix
        //
        // D3DMATRIX	m;
        // m._11 = world_to_camera_matrix.m11;
        // m._12 = world_to_camera_matrix.m12;
        // m._13 = world_to_camera_matrix.m13;
        // m._14 = 0.0f;
        //
        // m._21 = world_to_camera_matrix.m21;
        // m._22 = world_to_camera_matrix.m22;
        // m._23 = world_to_camera_matrix.m23;
        // m._24 = 0.0f;
        //
        // m._31 = world_to_camera_matrix.m31;
        // m._32 = world_to_camera_matrix.m32;
        // m._33 = world_to_camera_matrix.m33;
        // m._34 = 0.0f;
        //
        // m._41 = world_to_camera_matrix.tx;
        // m._42 = world_to_camera_matrix.ty;
        // m._43 = world_to_camera_matrix.tz;
        // m._44 = 1.0f;
        //
        // // Tell D3D about it
        //
        // HRESULT result = pD3DDevice->SetTransform(D3DTS_VIEW, &m);
        // assert(SUCCEEDED(result));
        // }

        // The model->clip matrix must be recomputed, next time we need it
        NEED_TO_COMPUTE_MODEL_TO_CLIP_MATRIX
            .lock()
            .unwrap()
            .need_to_compute_model_to_clip_matrix = true;
    }

    pub fn renderTriMesh(&self, p0: &Vec<RenderVertex>, p1: &i32, p2: &Vec<RenderTri>, p3: &i32) {
        todo!()
    }
}

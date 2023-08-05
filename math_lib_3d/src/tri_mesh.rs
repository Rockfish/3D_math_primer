#![allow(dead_code)]

use crate::aabb3::*;
use crate::renderer::*;

struct TriMesh {
    // Mesh data
    vertex_count: i32,
    vertex_list: RenderVertex, // ref?
    tri_count: i32,
    tri_list: RenderTri,

    // Axially aligned bounding box.  You must call computeBoundingBox()
    // to update this if you modify the vertex list directly
    bounding_box: AABB3,
}

impl TriMesh {}

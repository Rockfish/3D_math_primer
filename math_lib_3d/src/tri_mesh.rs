#![allow(dead_code)]

use crate::aabb3::*;
use crate::config::Config;
use crate::renderer::*;

/////////////////////////////////////////////////////////////////////////////
//
// 3D Math Primer for Games and Graphics Development
//
// TriMesh.cpp - Triangle mesh class for real-time use (rendering and
// collision detection)
//
// Visit gamemath.com for the latest version of this file.
//
/////////////////////////////////////////////////////////////////////////////

pub struct TriMesh {
    // Mesh data
    vertexCount: i32, //
    vertexList: Vec<RenderVertex>,
    triCount: i32,
    triList: Vec<RenderTri>,

    // Axially aligned bounding box.  You must call computeBoundingBox()
    // to update this if you modify the vertex list directly
    bounding_box: AABB3,
}

impl TriMesh {

// TriMesh
//
// Constructor - reset internal variables to default (empty) state

    pub fn default() -> Self {
        let mut bounding_box = AABB3::new();
        bounding_box.empty();
        TriMesh {
            vertexCount: 0,
            vertexList: Vec::new(),
            triCount: 0,
            triList: Vec::new(),
            bounding_box
        }
    }

    /*
    //---------------------------------------------------------------------------
    // allocateMemory
    //
    // Allocate mesh lists
        void	allocateMemory(int nVertexCount, int nTriCount) {

        // First, make sure and free any memory already allocated

        freeMemory();

        // !KLUDGE! Since we are using unsigned shorts for indices,
        // we can't handle meshes with more than 65535 vertices

        if (nVertexCount > 65536) {
        ABORT("Can't allocate triangle mesh with more than 655356 vertices");
        }

        // Allocate vertex list

        vertexCount = nVertexCount;
        vertexList = new RenderVertex[vertexCount];

        // Allocate triangle list

        triCount = nTriCount;
        triList = new RenderTri[triCount];
        }

    //---------------------------------------------------------------------------
    // freeMemory
    //
    // Free up any memory and reset object to default state

        void	freeMemory() {

        // Free lists

        delete [] vertexList;
        delete [] triList;

        // Reset variables

        vertexList = NULL;
        triList = NULL;
        vertexCount = 0;
        triCount = 0;
        }
    */
//---------------------------------------------------------------------------
// render
//
// Render the mesh using current 3D renderer context

    pub fn render(&self, config: &Config) {
        config.renderer.renderTriMesh(
            &self.vertexList,
            &self.vertexCount,
            &self.triList,
            &self.triCount
        );
}

//---------------------------------------------------------------------------
// computeBoundingBox
//
// Compute axially aligned bounding box from vertex list

pub fn computeBoundingBox(&self) {

// Empty bounding box
self.boundingBox.empty();

// Add in vertex locations
    for v in self.vertexList {
        self.bounding_box.add(v.p.clone());
    }
}

//---------------------------------------------------------------------------
// fromEditMesh
//
// Convert an EditTriMesh to a TriMesh.  Note that this function may need
// to make many logical changes to the mesh, such as ordering of vertices.
// Vertices may need to be duplicated to place UV's at the vertex level.
// Unused vertices are discarded and the vertex list order is optimized.
// However, the actual mesh geometry will not be modified as far as number
// of faces, vertex positions, vertex normals, etc.
//
// Also, since TriMesh doesn't have any notion of parts or materials,
// that information is lost.
//
// The input mesh is not modified.

pub fn fromEditMesh(mesh: &EditTriMesh) {
int	i;

// Make a copy of the mesh

EditTriMesh tempMesh(mesh);

// Make sure UV's are perperly set at the vertex level

tempMesh.copyUvsIntoVertices();

// Optimize the order of the vertices for best cache performance.
// This also discards unused vertices

tempMesh.optimizeVertexOrder();

// Allocate memory

allocateMemory(tempMesh.vertexCount(), tempMesh.triCount());

// Make sure we have something

if (triCount < 1) {
return;
}

// Convert vertices

for (i = 0 ; i < vertexCount ; ++i) {
const EditVertex *s = &tempMesh.vertex(i);
RenderVertex *d = &vertexList[i];

d->p = s->p;
d->n = s->normal;
d->u = s->u;
d->v = s->v;
}

// Convert faces

for (i = 0 ; i < triCount ; ++i) {
const EditTri *s = &tempMesh.tri(i);
RenderTri *d = &triList[i];
d->index[0] = s->v[0].index;
d->index[1] = s->v[1].index;
d->index[2] = s->v[2].index;
}

// Make sure bounds are computed

computeBoundingBox();
}

//---------------------------------------------------------------------------
// toEditMesh
//
// Convert a TriMesh to an EditTriMesh.  The output mesh is setup with a
// single default part and a single default material.

void	toEditMesh(EditTriMesh &mesh) const {
// !FIXME!
assert(false);
}


}

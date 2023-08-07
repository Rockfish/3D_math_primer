#![allow(dead_code)]

use crate::matrix4x3::Matrix4x3;
use crate::vector3::Vector3;
use debug_print::debug_println;

#[derive(Clone, Debug)]
pub struct EditTriMesh {
    // The mesh lists
    // vAlloc: f32,
    // vCount: f32,
    pub vList: Vec<Vertex>,

    // tAlloc: i32,
    // tCount: i32,
    pub tList: Vec<Tri>,

    // mCount: i32,
    pub mList: Vec<Material>,

    // pCount: i32,
    pub pList: Vec<Part>,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    // 3D vertex position;
    pub p: Vector3,

    // Vertex-level texture mapping coordinates.  Notice that
    // these may be invalid at various times.  The "real" UVs
    // are in the triangles.  For rendering, we often need UV's
    // at the vertex level.  But for many other optimizations,
    // we may need to weld vertices for faces with different
    // UV's.
    pub u: f32,
    pub v: f32,

    // vertex-level surface normal.  Again, this is only
    // valid in certain circumstances
    pub normal: Vector3,

    // Utility "mark" variable, often handy
    pub mark: i32,
}

#[derive(Clone, Debug)]
pub struct Vert {
    pub index: usize,
    // index into the vertex list
    // mapping coordinates
    pub u: f32,
    pub v: f32,
}

#[derive(Clone, Debug)]
pub struct Tri {
    // Face vertices.
    pub v: [Vert; 3],

    // Surface normal
    pub normal: Vector3,

    // Which part does this tri belong to?
    pub part: usize,

    // Index into the material list
    pub material: usize,

    // Utility "mark" variable, often handy
    pub mark: i32,
}

#[derive(Clone, Debug)]
pub struct Material {
    pub diffuseTextureName: String,
    // Utility "mark" variable, often handy
    pub mark: i32,
}

#[derive(Clone, Debug)]
pub struct Part {
    name: String,
    // Utility "mark" variable, often handy
    mark: i32,
}

#[derive(Clone, Debug)]
pub struct OptimizationParameters {
    // A tolerance value which is used to
    // determine if two vertices are coincident.
    coincidentVertexTolerance: f32,

    // Triangle angle tolerance.  Vertices
    // are not welded if the are on an edge
    // and the angle between the nrmals of the
    // triangles on this edge are too
    // far apart.  We store the cosine of this
    // value since that's what's actually used.
    // Use the functions to set it
    cosOfEdgeAngleTolerance: f32,
}

impl Vertex {
    //---------------------------------------------------------------------------
    // Vertex::setDefaults
    //
    // Reset vertex to a default state

    pub fn default() -> Self {
        Vertex {
            p: Vector3::identity(),
            u: 0.0,
            v: 0.0,
            normal: Vector3::identity(),
            mark: 0,
        }
    }
}

impl Vert {
    pub fn default() -> Self {
        Vert {
            index: 0,
            u: 0.0,
            v: 0.0,
        }
    }
}

impl Tri {
    pub fn default() -> Self {
        Tri {
            v: [Vert::default(), Vert::default(), Vert::default()],
            normal: Vector3::identity(),
            part: 0,
            material: 0,
            mark: 0,
        }
    }

    //---------------------------------------------------------------------------
    // Tri::isDegenerate
    //
    // Return true if we are degenerate (any two vertex indices are the same)

    pub fn isDegenerate(&self) -> bool {
        return (self.v[0].index == self.v[1].index)
            || (self.v[1].index == self.v[2].index)
            || (self.v[0].index == self.v[2].index);
    }

    //---------------------------------------------------------------------------
    // Tri::findVertex
    //
    // Check if we use the vertex (by index into the master vertex list).  Return
    // the first face vertex index (0..2) if we reference it, or -1 otherwise

    pub fn findVertex(&self, vertexIndex: usize) -> i32 {
        // Search vertices.  Let's unroll the loop here...
        if self.v[0].index == vertexIndex {
            return 0;
        }
        if self.v[1].index == vertexIndex {
            return 1;
        }
        if self.v[2].index == vertexIndex {
            return 2;
        }

        // Not found.
        -1
    }
}

impl Material {
    pub fn default() -> Material {
        Material {
            diffuseTextureName: "".to_string(),
            mark: 0,
        }
    }
}

impl Part {
    pub fn default() -> Part {
        Part {
            name: "".to_string(),
            mark: 0,
        }
    }
}

impl OptimizationParameters {
    pub fn default() -> OptimizationParameters {
        OptimizationParameters {
            // Weld vertices within 1/8 of an inch.  (We use 1 unit = 1ft)
            coincidentVertexTolerance: 1.0 / 12.0 / 8.0,

            // Weld vertices across edge if the edge is 80 degrees or more.
            // If more (for example, the edges of a cube) then let's keep
            // the edges detached
            cosOfEdgeAngleTolerance: 80.0,
        }
    }

    //---------------------------------------------------------------------------
    // OptimizationParameters::setEdgeAngleToleranceInDegrees
    //
    // Set tolerance angle value used to determine if two vertices can be
    // welded.  If they share a very "sharp" edge we may not wish to weld them,
    // since it destroys the lighting discontinuity that should be present at
    // this geometric discontinuity.
    //
    // Pass in a really large number (> 180 degrees) to effectively
    // weld all vertices, regardless of angle tolerance
    pub fn setEdgeAngleToleranceInDegrees(&mut self, degrees: f32) {
        // Check for a really big angle

        if degrees >= 180.0 {
            // Slam cosine to very small number

            self.cosOfEdgeAngleTolerance = -999.0;
        } else {
            // Compute the cosine
            self.cosOfEdgeAngleTolerance = degrees.to_radians().cos();
        }
    }
}

impl EditTriMesh {
    pub fn default() -> EditTriMesh {
        EditTriMesh {
            vList: vec![],
            tList: vec![],
            mList: vec![],
            pList: vec![],
        }
    }

    /////////////////////////////////////////////////////////////////////////////
    //
    // EditTriMesh members - Accessors to the mesh data
    //
    // All of these functions act like an array operator, returning a reference
    // to the element.  They provide array bounds checking, and can therefore
    // catch a number of common bugs.  We have and non-versions,
    // so you can only modify a non-mesh.
    //
    /////////////////////////////////////////////////////////////////////////////
    //
    // pub fn vertexes(&self, vertexIndex: i32) -> Vertex {
    //     // assert(vertexIndex >= 0);
    //     // assert(vertexIndex < vCount);
    //     return self.vList[vertexIndex as usize];
    // }
    //
    // pub fn triangles(&self, triIndex: i32) -> Tri {
    //     // assert(triIndex >= 0);
    //     // assert(triIndex < tCount);
    //     return self.tList[triIndex];
    // }
    //
    // pub fn materials(&self, materialIndex: i32) -> Material {
    //     // assert(materialIndex >= 0);
    //     // assert(materialIndex < mCount);
    //     return self.mList[materialIndex];
    // }
    //
    // pub fn parts(&self, partIndex: i32) -> Part {
    //     // assert(partIndex >= 0);
    //     // assert(partIndex < pCount);
    //     return self.pList[partIndex];
    // }

    /////////////////////////////////////////////////////////////////////////////
    //
    // EditTriMesh members - Basic mesh operations
    //
    // All of these functions act like an array operator, returning a reference
    // to the element.  They provide array bounds checking, and can therefore
    // catch a number of common bugs.  We have and non-versions,
    // so you can only modify a non-mesh.
    //
    /////////////////////////////////////////////////////////////////////////////

    //---------------------------------------------------------------------------
    // empty
    //
    // Reset the mesh to empty state
    pub fn empty(&mut self) {
        self.vList.clear();
        self.tList.clear();
        self.mList.clear();
        self.pList.clear();
    }

    /* not sure this applies to Rust
    //---------------------------------------------------------------------------
    // setVertexCount
    //
    // Set the vertex count.  If the list is grown, the new vertices at the end
    // are initialized with default values.  If the list is shrunk, any invalid
    // faces are deleted.

    pub fn setVertexCount(int vc) {
    assert(vc >= 0);

    // Make sure we had enough allocated coming in

    assert(vCount <= vAlloc);

    // Check if growing or shrinking the list

    if (vc > vCount) {

    // Check if we need to allocate more

    if (vc > vAlloc) {

    // We need to grow the list.  Figure out the
    // new count.  We don't want to constantly be
    // allocating memory every time a single vertex
    // is added, but yet we don't want to allocate
    // too much memory and be wasteful.  The system
    // shown below seems to be a good compromise.

    vAlloc = vc * 4 / 3 + 10;
    vList = (Vertex *)::realloc(vList, vAlloc * sizeof(*vList));

    // Check for out of memory.  You may need more
    // robust error handling...

    if (vList == NULL) {
    ABORT("Out of memory");
    }
    }

    // Initilaize the new vertices

    while (vCount < vc) {
    vList[vCount].setDefaults();
    ++vCount;
    }

    } else if (vc < vCount) {

    // Shrinking the list.  Go through
    // and mark invalid faces for deletion

    for (int i = 0 ; i < triCount() ; ++i) {
    Tri *t = &tri(i);
    if (
    (t->v[0].index >= vc) ||
    (t->v[1].index >= vc) ||
    (t->v[2].index >= vc)
    ) {

    // Mark it for deletion

    t->mark = 1;

    } else {

    // It's OK

    t->mark = 0;
    }
    }

    // Delete the marked triangles

    deleteMarkedTris(1);

    // Set the new count.  Any extra memory is
    // wasted for now...

    vCount = vc;
    }

    }


    //---------------------------------------------------------------------------
    // setTriCount
    //
    // Set the triangle count.  If the list is grown, the new triangles at the
    // end are initialized with default values.

    pub fn setTriCount(int tc) {
    assert(tc >= 0);

    // Make sure we had enough allocated coming in

    assert(tCount <= tAlloc);

    // Check if we are growing the list

    if (tc > tCount) {

    // Check if we need to allocate more

    if (tc > tAlloc) {

    // We need to grow the list.  Figure out the
    // new count.  We don't want to constantly be
    // allocating memory every time a single tri
    // is added, but yet we don't want to allocate
    // too much memory and be wasteful.  The system
    // shown below seems to be a good compromise.

    tAlloc = tc * 4 / 3 + 10;
    tList = (Tri *)::realloc(tList, tAlloc * sizeof(*tList));

    // Check for out of memory.  You may need more
    // robust error handling...

    if (tList == NULL) {
    ABORT("Out of memory");
    }
    }

    // Initilaize the new triangles

    while (tCount < tc) {
    tList[tCount].setDefaults();
    ++tCount;
    }
    } else {

    // Set the new count.  Any extra memory is
    // wasted for now...

    tCount = tc;
    }
    }

    //---------------------------------------------------------------------------
    // setMaterialCount
    //
    // Set the material count.  If the list is grown, the new materials at the end
    // are initialized with default values.  If the list is shrunk, any invalid
    // faces are deleted.

    pub fn setMaterialCount(int mc) {
    assert(mc >= 0);

    // Check if growing or shrinking the list

    if (mc > mCount) {

    // Grow the list.  For materials, we don't have any fancy
    // allocation like we do for the vertices and triangles.

    mList = (Material *)::realloc(mList, mc * sizeof(*mList));

    // Check for out of memory.  You may need more
    // robust error handling...

    if (mList == NULL) {
    ABORT("Out of memory");
    }

    // Initilaize the new materials

    while (mCount < mc) {
    mList[mCount].setDefaults();
    ++mCount;
    }

    } else if (mc < mCount) {

    // Shrinking the list.  Go through
    // and mark invalid faces for deletion

    for (int i = 0 ; i < triCount() ; ++i) {
    Tri *t = &tri(i);
    if (t->material >= mc) {

    // Mark it for deletion

    t->mark = 1;

    } else {

    // It's OK

    t->mark = 0;
    }
    }

    // Delete the marked triangles

    deleteMarkedTris(1);

    // Set the new count.  For now, no need to
    // shrink the list.  We'll just waste it.

    mCount = mc;
    }

    }

    //---------------------------------------------------------------------------
    // setPartCount
    //
    // Set the part count.  If the list is grown, the new parts at the end
    // are initialized with default values.  If the list is shrunk, any invalid
    // faces are deleted.

    pub fn setPartCount(int pc) {
    assert(pc >= 0);

    // Check if growing or shrinking the list

    if (pc > pCount) {

    // Grow the list.  For parts, we don't have any fancy
    // allocation like we do for the vertices and triangles.

    pList = (Part *)::realloc(pList, pc * sizeof(*pList));

    // Check for out of memory.  You may need more
    // robust error handling...

    if (pList == NULL) {
    ABORT("Out of memory");
    }

    // Initilaize the new parts

    while (pCount < pc) {
    pList[pCount].setDefaults();
    ++pCount;
    }

    } else if (pc < pCount) {

    // Shrinking the list.  Go through
    // and mark invalid faces for deletion

    for (int i = 0 ; i < triCount() ; ++i) {
    Tri *t = &tri(i);
    if (t->part >= pc) {

    // Mark it for deletion

    t->mark = 1;

    } else {

    // It's OK

    t->mark = 0;
    }
    }

    // Delete the marked triangles

    deleteMarkedTris(1);

    // Set the new count.  For now, no need to
    // shrink the list.  We'll just waste it.

    pCount = pc;
    }

    }
         */

    //---------------------------------------------------------------------------
    // addTri
    //
    // Add a new, default triangle.  The index of the new item is returned

    pub fn addDefaultTri(&mut self) -> i32 {
        self.tList.push(Tri::default());
        (self.tList.len() - 1) as i32
    }

    pub fn addTri(&mut self, tri: Tri) -> i32 {
        self.tList.push(tri);
        (self.tList.len() - 1) as i32
    }

    pub fn addDefaultVertex(&mut self) -> i32 {
        self.vList.push(Vertex::default());
        (self.vList.len() - 1) as i32
    }

    pub fn addVertex(&mut self, vertex: Vertex) -> i32 {
        self.vList.push(vertex);
        (self.vList.len() - 1) as i32
    }

    //---------------------------------------------------------------------------
    // dupVertex
    //
    // Add a duplicate of a vertex to the end of the list.
    pub fn dupVertex(&mut self, srcVertexIndex: i32) -> i32 {
        let dup = self.vList[srcVertexIndex as usize].clone();
        self.vList.push(dup);
        (self.vList.len() - 1) as i32
    }

    //---------------------------------------------------------------------------
    // addMaterial
    //
    // Add a material to the end of the list.
    pub fn addMaterial(&mut self, m: Material) -> i32 {
        self.mList.push(m);
        (self.mList.len() - 1) as i32
    }

    //---------------------------------------------------------------------------
    // addPart
    //
    // Add a part to the end of the list.
    pub fn addPart(&mut self, p: Part) -> i32 {
        self.pList.push(p);
        (self.pList.len() - 1) as i32
    }

    //---------------------------------------------------------------------------
    // markAllVertices
    //
    // Mark all vertices with the given value
    pub fn markAllVertices(&mut self, mark: i32) {
        for v in self.vList.iter_mut() {
            v.mark = mark;
        }
    }

    //---------------------------------------------------------------------------
    // markAllTris
    //
    // Mark all triangles with the given value
    pub fn markAllTris(&mut self, mark: i32) {
        for t in self.tList.iter_mut() {
            t.mark = mark;
        }
    }

    //---------------------------------------------------------------------------
    // markAllMaterials
    //
    // Mark all materials with the given value
    pub fn markAllMaterials(&mut self, mark: i32) {
        for m in self.mList.iter_mut() {
            m.mark = mark;
        }
    }

    //---------------------------------------------------------------------------
    // markAllParts
    //
    // Mark all parts with the given value
    pub fn markAllParts(&mut self, mark: i32) {
        for p in self.pList.iter_mut() {
            p.mark = mark;
        }
    }

    //---------------------------------------------------------------------------
    // deleteVertex
    //
    // Deletes one vertex from the vertex list.  This will fixup vertex
    // indices in the triangles, and also delete triangles that referenced
    // that vertex

    pub fn deleteVertex(&mut self, vertexIndex: usize) {
        // Check index.  Warn in debug build, don't crash release
        if (vertexIndex < 0) || (vertexIndex >= self.vList.len()) {
            debug_assert!(
                false,
                "{}",
                format!("vertexIndex out of range: {}", vertexIndex)
            );
            return;
        }

        // Scan triangle list and fixup vertex indices
        for t in self.tList.iter_mut() {
            for v in t.v.iter_mut() {
                // mark Tri for deletion is it uses the vert that is going to be deleted
                if v.index == vertexIndex {
                    t.mark = 1;
                    break;
                }
                // Fix up the vert indexes of vertexes after the deleted vertex
                if v.index > vertexIndex {
                    v.index -= 1;
                }
            }
        }

        self.vList.remove(vertexIndex as usize);

        self.deleteMarkedTris(1);
    }

    //---------------------------------------------------------------------------
    // deleteTri
    //
    // Deletes one triangle from the triangle list.

    pub fn deleteTri(&mut self, triIndex: i32) {
        // Check index.  Warn in debug build, don't crash release
        if (triIndex < 0) || (triIndex >= self.vList.len() as i32) {
            debug_assert!(false, "{}", format!("triIndex out of range: {}", triIndex));
            return;
        }

        // Delete it
        self.tList.remove(triIndex as usize);
    }

    //---------------------------------------------------------------------------
    // deleteMaterial
    //
    // Deletes one material from the material list.  Material indices in
    // the triangles are fixed up and any triangles that used that material
    // are deleted

    pub fn deleteMaterial(&mut self, materialIndex: usize) {
        // Check index.  Warn in debug build, don't crash release
        if (materialIndex < 0) || (materialIndex >= self.vList.len()) {
            debug_assert!(
                false,
                "{}",
                format!("materialIndex out of range: {}", materialIndex)
            );
            return;
        }

        // Scan triangle list and fixup material indices
        for tri in self.tList.iter_mut() {
            if tri.material == materialIndex {
                tri.mark = 1;
            } else {
                tri.mark = 0;
                if tri.material > materialIndex {
                    tri.material -= 1;
                }
            }
        }

        self.mList.remove(materialIndex as usize);

        self.deleteMarkedTris(1);
    }

    //---------------------------------------------------------------------------
    // deletePart
    //
    // Deletes one part from the part list.  Part indices in the triangles are
    // fixed up and any triangles from that part are deleted

    pub fn deletePart(&mut self, partIndex: usize) {
        // Check index.  Warn in debug build, don't crash release
        if (partIndex < 0) || (partIndex >= self.vList.len()) {
            debug_assert!(
                false,
                "{}",
                format!("partIndex out of range: {}", partIndex)
            );
            return;
        }

        // Scan triangle list and fixup part indices
        for tri in self.tList.iter_mut() {
            if tri.part == partIndex {
                tri.mark = 1;
            } else {
                tri.mark = 0;
                if tri.material > partIndex {
                    tri.material -= 1;
                }
            }
        }

        self.pList.remove(partIndex as usize);

        self.deleteMarkedTris(1);
    }

    //---------------------------------------------------------------------------
    // deleteUnusedMaterials
    //
    // Scan list of materials and delete any that are not used by any triangles
    //
    // This method may seem a little more complicated, but it operates
    // in linear time with respect to the number of triangles.
    // Other methods will run in quadratic time or worse.

    pub fn deleteUnusedMaterials(&mut self) {
        // Assume all materials will be unused
        self.markAllMaterials(0);

        // Scan triangle list and mark referenced materials
        for tri in self.tList.iter_mut() {
            self.mList[tri.material].mark = 1;
        }

        // OK, figure out how many materials there will be,
        // and where they will go int he new material list,
        // after the unused ones are removed

        let initial_material_count = self.mList.len();
        let mut new_material_count: usize = 0;

        for m in self.mList.iter_mut() {
            // Was it used?
            if m.mark == 0 {
                // No - mark it to be whacked
                m.mark = -1;
            } else {
                m.mark = new_material_count as i32;
                new_material_count += 1;
            }
        }

        // Check if nothing got deleted, then don't bother with the
        // rest of this
        if new_material_count == self.mList.len() {
            return;
        }

        // Fixup indices in the face list
        for tri in self.tList.iter_mut() {
            tri.material = self.mList[tri.material].mark as usize;
        }

        // Remove the empty spaces from the material list

        let extracted_material_count = self.mList.extract_if(|m| -> bool { m.mark == -1 }).count();

        /*
        let mut dest_material_index = 0;
        for (i, mut m) in self.mList.iter().enumerate() {
            if m.mark != -1 {
                debug_assert!(m.mark == dest_material_index, "{}",format!("dest_material_index does not match mark."));
                if i != dest_material_index as usize {
                    // Todo: does this work as expected?
                    self.mList[dest_material_index as usize] = *m;
                }
                dest_material_index += 1;
            }
        }
         */

        assert_eq!(
            initial_material_count - extracted_material_count,
            new_material_count,
            "{}",
            format!(
                "initial_material_count - extracted_material_count: '{}' and new_material_count: '{}' should match.",
                extracted_material_count, new_material_count
            )
        );

        // Remove the extra entries at the end of the list
        // todo: verify the trimming of the tail of the material list after deletion of marked materials
        // for i in (dest_material_index + 1)..self.mList.len() {
        //    self.mList.remove(i as usize);
        // }
    }

    //---------------------------------------------------------------------------
    // deleteEmptyParts
    //
    // Scan list of parts and delete any that do not contain any triangles
    //
    // This method may seem a little more complicated, but it operates
    // in linear time with respect to the number of triangles.
    // Other methods will run in quadratic time or worse.

    pub fn deleteEmptyParts(&mut self) {
        // Assume all parts will be empty
        self.markAllParts(0);

        // Scan triangle list and mark referenced parts
        for tri in self.tList.iter_mut() {
            self.pList[tri.part].mark = 1;
        }

        // OK, figure out how many parts there will be,
        // and where they will go int he new part list,
        // after the unused ones are removed

        let initial_part_count = self.pList.len();
        let mut new_part_count: usize = 0;

        for p in self.pList.iter_mut() {
            // Was it used?
            if p.mark == 0 {
                // No - mark it to be whacked
                p.mark = -1;
            } else {
                p.mark = new_part_count as i32;
                new_part_count += 1;
            }
        }

        // Check if nothing got deleted, then don't bother with the
        // rest of this
        if new_part_count == self.pList.len() {
            return;
        }

        // Fixup indices in the face list
        for tri in self.tList.iter_mut() {
            tri.part = self.pList[tri.part].mark as usize;
        }

        // Remove the empty spaces from the part list

        let extracted_count = self.pList.extract_if(|p| -> bool { p.mark == -1 }).count();

        //let mut destPartIndex: usize = 0;
        //
        // for (i, mut p) in self.pList.iter().enumerate() {
        //     if p.mark != -1 {
        //         debug_assert!(p.mark == destPartIndex as i32, "{}", format!("destPartIndex does not match mark."));
        //         if i != destPartIndex as usize {
        //             self.pList[destPartIndex] = p;
        //         }
        //         destPartIndex += 1;
        //     }
        // }
        //
        assert_eq!(
            initial_part_count - extracted_count,
            new_part_count,
            "{}",
            format!(
                "initial_part_count - extracted_count: '{}' and new_part_count: '{}' should match.",
                extracted_count, new_part_count
            )
        );

        // Remove the extra entries at the end of the list
        // todo: verify the trimming of the tail of the material list after deletion of marked materials
        // for i in (new_part_count + 1)..self.pList.len() {
        //     self.mList.remove(i as usize);
        // }
    }

    //---------------------------------------------------------------------------
    // deleteMarkedTris
    //
    // Scan triangle list, deleting triangles with the given mark

    pub fn deleteMarkedTris(&mut self, mark: i32) {
        // Scan triangle list, and move triangles forward to
        // suck up the "holes" left by deleted triangles
        let extracted_count = self
            .tList
            .extract_if(|t| -> bool { t.mark == mark })
            .count();
        debug_println!("deleted tri count: {}", extracted_count);
    }

    //---------------------------------------------------------------------------
    // deleteDegenerateTris
    //
    // Scan triangle list and remove "degenerate" triangles.  See
    // isDegenerate() for the definition of "degenerate" in this case.
    pub fn deleteDegenerateTris(&mut self) {
        let extracted_count = self
            .tList
            .extract_if(|t| -> bool { t.isDegenerate() })
            .count();
        debug_println!("deleted degenerate tri count: {}", extracted_count);
    }

    //---------------------------------------------------------------------------
    // detachAllFaces
    //
    // Detach all the faces from one another. This creates a new vertex list,
    // with each vertex only used by one triangle. Simultaneously, unused
    // vertices are removed.
    pub fn detachAllFaces(&mut self) {
        // Check if we don't have any faces, then bail now.
        // This saves us a crash with a spurrious "out of memory"
        if self.tList.is_empty() {
            return;
        }

        // Figure out how many triangles we'll have

        let newVertexCount = self.tList.len() * 3;

        // Allocate a new vertex list
        let mut newVertexList: Vec<Vertex> = Vec::with_capacity(newVertexCount);

        for i in 0..newVertexCount {
            newVertexList.push(Vertex::default());
        }

        // Scan the triangle list and fill it in
        for (i, t) in self.tList.iter_mut().enumerate() {
            // Process the three vertices on this face
            for j in 0..3 {
                // Get source and destination vertex indices
                let s_index = t.v[j].index;
                let d_index = i * 3 + j;

                let new_v = &mut newVertexList[d_index];
                let old_v: &Vertex = &self.vList[s_index];

                // Copy the vertex
                new_v.p.copy(&old_v.p);
                new_v.normal.copy(&old_v.normal);
                new_v.u = old_v.u;
                new_v.v = old_v.v;

                t.v[j].index = d_index;
            }
        }

        // Install the new one
        self.vList = newVertexList;
    }

    //---------------------------------------------------------------------------
    // transformVertices
    //
    // Transform all the vertices.  We could transform the surface normals,
    // but they may not even be valid, anyway.  If you need them, compute them.
    pub fn transformVertices(&mut self, m: &Matrix4x3) {
        for vertex in self.vList.iter_mut() {
            vertex.p *= m;
        }
    }

    //---------------------------------------------------------------------------
    // extractParts
    //
    // Extract each part into a seperate mesh.  Each resulting mesh will
    // have exactly one part

    pub fn extractParts(&mut self, meshes: &mut Vec<EditTriMesh>) {

    // !SPEED! This function will run in O(partCount * triCount).
    // We could optimize it somewhat by having the triangles sorted by
    // part.  However, any real optimization would be considerably
    // more complicated.  Let's just keep it simple.

    // Scan through each part
/*
        for dMesh in meshes.iter_mut() {
            self.markAllVertices(-1);
            self.markAllMaterials(-1);


            dMesh.empty();
            dMesh.setPartCount(1);
            dMesh.pList = self.pList
        }

    for (int partIndex = 0 ; partIndex < partCount() ; ++partIndex) {

    // Get shortcut to destination mesh

    EditTriMesh *dMesh = &meshes[partIndex];

    // Mark all vertices and materials, assuming they will
    // not be used by this part

    markAllVertices(-1);
    markAllMaterials(-1);

    // Setup the destination part mesh with a single part

    dMesh->empty();
    dMesh->setPartCount(1);
    dMesh->part(0) = part(partIndex);

    // Convert face list, simultaneously building material and
    // vertex list

    for (int faceIndex = 0 ; faceIndex < triCount() ; ++faceIndex) {

    // Fetch shortcut, make sure it belongs to this
    // part

    Tri *tPtr = &tri(faceIndex);
    if (tPtr->part != partIndex) {
    continue;
    }

    // Make a copy

    Tri t = *tPtr;

    // Remap material index

    Material *m = &material(t.material);
    if (m->mark < 0) {
    m->mark = dMesh->addMaterial(*m);
    }
    t.material = m->mark;

    // Remap vertices

    for (int j = 0 ; j < 3 ; ++j) {
    Vertex *v = &vertex(t.v[j].index);
    if (v->mark < 0) {
    v->mark = dMesh->addVertex(*v);
    }
    t.v[j].index = v->mark;
    }

    // Add the face

    t.part = 0;
    dMesh->addTri(t);
    }
    }
        */
    }

    /*
    pub fn extractOnePartOneMaterial(int partIndex, int materialIndex, EditTriMesh *result) {

    // Mark all vertices, assuming they will not be used

    markAllVertices(-1);

    // Setup the destination mesh with a single part and material

    result->empty();
    result->setPartCount(1);
    result->part(0) = part(partIndex);
    result->setMaterialCount(1);
    result->material(0) = material(materialIndex);

    // Convert face list, simultaneously building vertex list

    for (int faceIndex = 0 ; faceIndex < triCount() ; ++faceIndex) {

    // Fetch shortcut, make sure it belongs to this
    // part and uses this material

    Tri *tPtr = &tri(faceIndex);
    if (tPtr->part != partIndex) {
    continue;
    }
    if (tPtr->material != materialIndex) {
    continue;
    }

    // Make a copy

    Tri t = *tPtr;

    // Remap vertices

    for (int j = 0 ; j < 3 ; ++j) {
    Vertex *v = &vertex(t.v[j].index);
    if (v->mark < 0) {
    v->mark = result->addVertex(*v);
    }
    t.v[j].index = v->mark;
    }

    // Add the face

    t.part = 0;
    t.material = 0;
    result->addTri(t);
    }
    }

    /////////////////////////////////////////////////////////////////////////////
    //
    // EditTriMesh members - Computations
    //
    /////////////////////////////////////////////////////////////////////////////

    //---------------------------------------------------------------------------
    // computeOneTriNormal
    //
    // Compute a single triangle normal.

    pub fn computeOneTriNormal(int triIndex) {
    computeOneTriNormal(tri(triIndex));
    }

    pub fn computeOneTriNormal(Tri &t) {

    // Fetch shortcuts to vertices

    Vector3 &v1 = vertex(t.v[0].index).p;
    Vector3 &v2 = vertex(t.v[1].index).p;
    Vector3 &v3 = vertex(t.v[2].index).p;

    // Compute clockwise edge vectors.  We use the edge vector
    // indexing that agrees with Section 12.6.

    Vector3 e1 = v3 - v2;
    Vector3 e2 = v1 - v3;

    // Cross product to compute surface normal

    t.normal = crossProduct(e1, e2);

    // Normalize it

    t.normal.normalize();
    }

    //---------------------------------------------------------------------------
    // computeTriNormals
    //
    // Compute all the triangle normals

    pub fn computeTriNormals() {
    for (int i = 0 ; i < triCount() ; ++i) {
    computeOneTriNormal(tri(i));
    }
    }

    //---------------------------------------------------------------------------
    // computeTriNormals
    //
    // Compute vertex level surface normals.  This automatically computes the
    // triangle level surface normals

    pub fn computeVertexNormals() {

    int	i;

    // First, make sure triangle level surface normals are up-to-date

    computeTriNormals();

    // Zero out vertex normals

    for (i = 0 ; i < vertexCount() ; ++i) {
    vertex(i).normal.zero();
    }

    // Sum in the triangle normals into the vertex normals
    // that are used by the triangle

    for (i = 0 ; i < triCount() ; ++i) {
    Tri *t = &tri(i);
    for (int j = 0 ; j < 3 ; ++j) {
    vertex(t->v[j].index).normal += t->normal;
    }
    }

    // Now "average" the vertex surface normals, by normalizing them

    for (i = 0 ; i < vertexCount() ; ++i) {
    vertex(i).normal.normalize();
    }
    }

    //---------------------------------------------------------------------------
    // computeBounds
    //
    // Compute the bounding box of the mesh

    AABB3	computeBounds() {

    // Generate the bounding box of the vertices

    AABB3	box;
    box.empty();
    for (int i = 0 ; i < vertexCount() ; ++i) {
    box.add(vertex(i).p);
    }

    // Return it

    return box;
    }

    /////////////////////////////////////////////////////////////////////////////
    //
    // EditTriMesh members - Optimization
    //
    /////////////////////////////////////////////////////////////////////////////

    //---------------------------------------------------------------------------
    // optimizeVertexOrder
    //
    // Re-order the vertex list, in the order that they are used by the faces.
    // This can improve cache performace and vertex caching by increasing the
    // locality of reference.
    //
    // If removeUnusedVertices is true, then any unused vertices are discarded.
    // Otherwise, they are retained at the end of the vertex list.  Normally
    // you will want to discard them, which is why we default the paramater to
    // true.

    pub fn optimizeVertexOrder(bool removeUnusedVertices) {

    int	i;

    // Mark all vertices with a very high mark, which assumes
    // that they will not be used

    for (i = 0 ; i < vertexCount() ; ++i) {
    vertex(i).mark = vertexCount();
    }

    // Scan the face list, and figure out where the vertices
    // will end up in the new, ordered list.  At the same time,
    // we remap the indices in the triangles according to this
    // new ordering.

    int	usedVertexCount = 0;
    for (i = 0 ; i < triCount() ; ++i) {
    Tri *t = &tri(i);

    // Process each of the three vertices on this triangle

    for (int j = 0 ; j < 3 ; ++j) {

    // Get shortcut to the vertex used

    Vertex *v = &vertex(t->v[j].index);

    // Has it been used already?

    if (v->mark == vertexCount()) {

    // We're the first triangle to use
    // this one.  Assign the vertex to
    // the next slot in the new vertex
    // list

    v->mark = usedVertexCount;
    ++usedVertexCount;
    }

    // Remap the vertex index

    t->v[j].index = v->mark;
    }
    }

    // Re-sort the vertex list.  This puts the used vertices
    // in order where they go, and moves all the unused vertices
    // to the end (in no particular order, since qsort is not
    // a stable sort)

    qsort(vList, vertexCount(), sizeof(Vertex), vertexCompareByMark);

    // Did they want to discard the unused guys?

    if (removeUnusedVertices) {

    // Yep - chop off the unused vertices by slamming
    // the vertex count.  We don't call the function to
    // set the vertex count here, since it will scan
    // the triangle list for any triangle that use those
    // vertices.  But we already know that all of the
    // vertices we are deleting are unused

    vCount = usedVertexCount;
    }
    }

    //---------------------------------------------------------------------------
    // sortTrisByMaterial
    //
    // Sort triangles by material.  This is VERY important for effecient
    // rendering

    pub fn sortTrisByMaterial() {

    // Put the current index into the "mark" field so we can
    // have a stable sort

    for (int i = 0 ; i < triCount() ; ++i) {
    tri(i).mark = i;
    }

    // Use qsort

    qsort(tList, triCount(), sizeof(Tri), triCompareByMaterial);
    }

    //---------------------------------------------------------------------------
    // weldVertices
    //
    // Weld coincident vertices.  For the moment, this disregards UVs and welds
    // all vertices that are within geometric tolerance

    pub fn weldVertices(OptimizationParameters &opt) {

    // !FIXME!

    }

    //---------------------------------------------------------------------------
    // copyUvsIntoVertices
    //
    // Ensure that the vertex UVs are correct, possibly duplicating
    // vertices if necessary

    pub fn copyUvsIntoVertices() {

    // Mark all vertices indicating thet their UV's are invalid

    markAllVertices(0);

    // Scan the faces, and shove in the UV's into the vertices

    for (int triIndex = 0 ; triIndex < triCount() ; ++triIndex) {
    Tri *triPtr = &tri(triIndex);
    for (int i = 0 ; i < 3 ; ++i) {

    // Locate vertex

    int	vIndex = triPtr->v[i].index;
    Vertex *vPtr = &vertex(vIndex);

    // Have we filled in the UVs for this vertex yet?

    if (vPtr->mark == 0) {

    // Nope.  Shove them in

    vPtr->u = triPtr->v[i].u;
    vPtr->v = triPtr->v[i].v;

    // Mark UV's as valid, and keep going

    vPtr->mark = 1;
    continue;
    }

    // UV's have already been filled in by another face.
    // Did that face have the same UV's as me?

    if (
    (vPtr->u == triPtr->v[i].u) &&
    (vPtr->v == triPtr->v[i].v)
    ) {

    // Yep - no need to change anything

    continue;
    }

    // OK, we can't use this vertex - somebody else already has
    // it "claimed" with different UV's.  First, we'll search
    // for another vertex with the same position.  Yikes -
    // this requires a linear search through the vertex list.
    // Luckily, this should not happen the majority of the time.

    bool	foundOne = false;
    for (int newIndex = 0 ; newIndex < vertexCount() ; ++newIndex) {
    Vertex *newPtr = &vertex(newIndex);

    // Is the position and normal correct?

    if (
    (newPtr->p != vPtr->p) ||
    (newPtr->normal != vPtr->normal)
    ) {
    continue;
    }

    // OK, this vertex is geometrically correct.
    // Has anybody filled in the UV's yet?

    if (newPtr->mark == 0) {

    // We can claim this one.

    newPtr->mark = 1;
    newPtr->u = triPtr->v[i].u;
    newPtr->v = triPtr->v[i].v;

    // Remap vertex index

    triPtr->v[i].index = newIndex;

    // No need to keep looking

    foundOne = true;
    break;
    }

    // Already claimed by somebody else, so we can't change
    // them.  but are they correct, already anyway?

    if (
    (newPtr->u == triPtr->v[i].u) &&
    (newPtr->v == triPtr->v[i].v)
    ) {

    // Yep - no need to change anything.  Just remap the
    // vertex index

    triPtr->v[i].index = newIndex;

    // No need to keep looking

    foundOne = true;
    break;
    }

    // No good - keep looking
    }

    // Did we find a vertex?

    if (!foundOne) {

    // Nope, we'll have to create a new one

    Vertex newVertex = *vPtr;
    newVertex.mark = 1;
    newVertex.u = triPtr->v[i].u;
    newVertex.v = triPtr->v[i].v;
    triPtr->v[i].index = addVertex(newVertex);
    }
    }
    }
    }

    // Do all of the optimizations and prepare the model
    // for fast rendering under *most* rendering systems,
    // with proper lighting.

    pub fn optimizeForRendering() {
    computeVertexNormals();
    }

    /////////////////////////////////////////////////////////////////////////////
    //
    // EditTriMesh members - Import/Export S3D format
    //
    // For more on the S3D file format, and free tools for using the
    // S3D format with popular rendering packages, visit
    // gamemath.com
    //
    /////////////////////////////////////////////////////////////////////////////

    //---------------------------------------------------------------------------
    // importS3d
    //
    // Load up an S3D file.  Returns true on success.  If failure, returns
    // false and puts an error message into returnErrMsg

    bool	importS3d(char *filename, char *returnErrMsg) {
    int	i;

    // Try to open up the file

    FILE *f = fopen(filename, "rt");
    if (f == NULL) {
    strcpy(returnErrMsg, "Can't open file");
    failed:
    empty();
    if (f != NULL) {
    fclose(f);
    }
    return false;
    }

    // Read and check version

    if (!skipLine(f)) {
    corrupt:
    strcpy(returnErrMsg, "File is corrupt");
    goto failed;
    }
    int	version;
    if (fscanf(f, "%d\n", &version) != 1) {
    goto corrupt;
    }
    if (version != 103) {
    sprintf(returnErrMsg, "File is version %d - only version 103 supported", version);
    goto failed;
    }

    // Read header

    if (!skipLine(f)) {
    goto corrupt;
    }
    int	numTextures, numTris, numVerts, numParts, numFrames, numLights, numCameras;
    if (fscanf(f, "%d , %d , %d , %d , %d , %d , %d\n", &numTextures, &numTris, &numVerts, &numParts, &numFrames, &numLights, &numCameras) != 7) {
    goto corrupt;
    }

    // Allocate lists

    setMaterialCount(numTextures);
    setTriCount(numTris);
    setVertexCount(numVerts);
    setPartCount(numParts);

    // Read part list.  the only number we care about
    // is the triangle count, which we'll temporarily
    // stach into the mark field

    if (!skipLine(f)) {
    goto corrupt;
    }
    int	firstVert = 0, firstTri = 0;
    for (i = 0 ; i < numParts ; ++i) {
    Part *p = &part(i);
    int	partFirstVert, partNumVerts, partFirstTri, partNumTris;
    if (fscanf(f, "%d , %d , %d , %d , \"%[^\"]\"\n", &partFirstVert, &partNumVerts, &partFirstTri, &partNumTris, p->name) != 5) {
    sprintf(returnErrMsg, "Corrupt at part %d", i);
    goto failed;
    }
    if (firstVert != partFirstVert || firstTri != partFirstTri) {
    sprintf(returnErrMsg, "Part vertex/tri mismatch detected at part %d", i);
    goto failed;
    }
    p->mark = partNumTris;
    firstVert += partNumVerts;
    firstTri += partNumTris;
    }
    if (firstVert != numVerts || firstTri != numTris) {
    strcpy(returnErrMsg, "Part vertex/tri mismatch detected at end of part list");
    goto failed;
    }

    // Read textures.

    if (!skipLine(f)) {
    goto corrupt;
    }
    for (i = 0 ; i < numTextures ; ++i) {
    Material	*m = &material(i);

    // Fetch line of text

    if (fgets(m->diffuseTextureName, sizeof(m->diffuseTextureName), f) != m->diffuseTextureName) {
    sprintf(returnErrMsg, "Corrupt reading texture %d", i);
    goto failed;
    }

    // Styrip off newline, which fgets leaves.
    // Wouldn't it have been nice if the stdio
    // functions would just have a function to read a line
    // WITHOUT the newline character.  What a pain...

    char *nl = strchr(m->diffuseTextureName, '\n');
    if (nl != NULL) {
    *nl = '\0';
    }
    }

    // Read triangles a part at a time

    if (!skipLine(f)) {
    goto corrupt;
    }
    int	whiteTextureIndex = -1;
    int	destTriIndex = 0;
    for (int partIndex = 0 ; partIndex < numParts ; ++partIndex) {

    // Read all triangles in this part

    for (int i = 0 ; i < part(partIndex).mark ; ++i) {

    // get shortcut to destination triangle

    Tri *t = &tri(destTriIndex);

    // Slam part number

    t->part = partIndex;

    // Parse values from file

    if (fscanf(f, "%d , %d , %f , %f , %d , %f , %f , %d , %f , %f\n",
    &t->material,
    &t->v[0].index, &t->v[0].u, &t->v[0].v,
    &t->v[1].index, &t->v[1].u, &t->v[1].v,
    &t->v[2].index, &t->v[2].u, &t->v[2].v
    ) != 10) {
    sprintf(returnErrMsg, "Corrupt reading triangle %d (%d of part %d)", destTriIndex, i, partIndex);
    goto failed;
    }

    // Check for untextured triangle

    if (t->material < 0) {
    if (whiteTextureIndex < 0) {
    Material whiteMaterial;
    strcpy(whiteMaterial.diffuseTextureName, "White");
    whiteTextureIndex = addMaterial(whiteMaterial);
    }
    t->material = whiteTextureIndex;
    }

    // Scale UV's to 0...1 range

    t->v[0].u /= 256.0f;
    t->v[0].v /= 256.0f;
    t->v[1].u /= 256.0f;
    t->v[1].v /= 256.0f;
    t->v[2].u /= 256.0f;
    t->v[2].v /= 256.0f;

    // Next triangle, please

    ++destTriIndex;
    }
    }
    assert(destTriIndex == triCount());

    // Read vertices

    if (!skipLine(f)) {
    goto corrupt;
    }
    for (i = 0 ; i < numVerts ; ++i) {
    Vertex *v = &vertex(i);
    if (fscanf(f, "%f , %f , %f\n", &v->p.x, &v->p.y, &v->p.z) != 3) {
    sprintf(returnErrMsg, "Corrupt reading vertex %d", i);
    goto failed;
    }
    }

    // OK, we don't need anything from the rest of the file.  Close file.

    fclose(f);
    f = NULL;

    // Check for structural errors in the mesh

    if (!validityCheck(returnErrMsg)) {
    goto failed;
    }

    // OK!

    return true;
    }

    pub fn exportS3d(char *filename) {
    }

    /////////////////////////////////////////////////////////////////////////////
    //
    // EditTriMesh members - Debugging
    //
    /////////////////////////////////////////////////////////////////////////////

    pub fn validityCheck() {
    char	errMsg[256];
    if (!validityCheck(errMsg)) {
    ABORT("EditTriMesh failed validity check:\n%s", errMsg);
    }
    }

    bool	validityCheck(char *returnErrMsg) {
    return true;
    }

    }

    //---------------------------------------------------------------------------
    // operator=
    //
    // Assignment operator - make a copy of the mesh

    EditTriMesh &operator=(EditTriMesh &src) {
    int	i;

    // Start by freeing up what we already have

    empty();

    // Copy materials and parts first.  We copy these stupidly,
    // since the lists probably won't be very big

    setMaterialCount(src.materialCount());
    for (i = 0 ; i < materialCount() ; ++i) {
    material(i) = src.material(i);
    }

    setPartCount(src.partCount());
    for (i = 0 ; i < partCount() ; ++i) {
    part(i) = src.part(i);
    }

    // Make sure vertex list isn't empty

    if (src.vertexCount() > 0) {

    // Compute size in bytes

    int	bytes = src.vertexCount() * sizeof(*vList);

    // Allocate it.  We don't use setVertexCount(), since
    // that initializes the list, which we don't need

    vList = (Vertex *)::malloc(bytes);
    if (vList == NULL) {
    ABORT("Out of memory");
    }
    vCount = src.vertexCount();
    vAlloc = vCount;

    // Blast copy it

    memcpy(vList, src.vList, bytes);
    }

    // Make sure face list isn't empty

    if (src.triCount() > 0) {

    // Compute size in bytes

    int	bytes = src.triCount() * sizeof(*tList);

    // Allocate it.  We don't use setVertexCount(), since
    // that initializes the list, which we don't need

    tList = (Tri *)::malloc(bytes);
    if (tList == NULL) {
    ABORT("Out of memory");
    }
    tCount = src.triCount();
    tAlloc = tCount;

    // Blast copy it

    memcpy(tList, src.tList, bytes);
    }

    // Return reference to l-value, as per C convention

    return *this;
    }



    /////////////////////////////////////////////////////////////////////////////
    //
    // Local utility stuff
    //
    /////////////////////////////////////////////////////////////////////////////

    //---------------------------------------------------------------------------
    // vertexCompareByMark
    //
    // Compare two vertices by their mark field.  Used to sort using qsort.

    static int vertexCompareByMark(void *va, void *vb) {

    // Cast pointers

    Vertex *a = (Vertex *)va;
    Vertex *b = (Vertex *)vb;

    // Return comparison result as per Qsort conventions:
    //
    // <0	a goes "before" b
    // >0	a goes "after" b
    // 0	doesn't matter
    //
    // We want the lower mark to come first

    return a->mark - b->mark;
    }

    //---------------------------------------------------------------------------
    // triCompareByMaterial
    //
    // Compare two triangles by their material field.  Used to sort using qsort.

    static int triCompareByMaterial(void *va, void *vb) {

    // Cast pointers

    Tri *a = (Tri *)va;
    Tri *b = (Tri *)vb;

    // Return comparison result as per Qsort conventions:
    //
    // <0	a goes "before" b
    // >0	a goes "after" b
    // 0	doesn't matter
    //
    // We want the lower material to come first

    if (a->material < b->material) return -1;
    if (a->material > b->material) return +1;

    // Same material - use "mark" field, which contained the
    // original index, so we'll have a stable sort

    return a->mark - b->mark;
    }

    //---------------------------------------------------------------------------
    // skipLine
    //
    // Skip a line of text from a file.  Returns false on failure (EOF or error).

    static bool	skipLine(FILE *f) {
    for (;;) {
    int c = fgetc(f);
    if (c < 0) {
    return false;
    }
    if (c == '\n') {
    return true;
    }
    }

     */
}

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::config::Config;
use crate::edit_tri_mesh::EditTriMesh;
use crate::renderer::TextureReference;
use crate::s3d_handler::import_s3d;
use crate::tri_mesh::TriMesh;

pub struct Model {
    pub partCount: usize,
    pub partMeshList: Vec<TriMesh>,
    pub partTextureList: Vec<TextureReference>,
}

impl Model {
    pub fn new(config: &Config) -> Model {
        Model {
            partCount: 0,
            partMeshList: vec![],
            partTextureList: vec![],
        }
    }

    //---------------------------------------------------------------------------
    // allocateMemory
    //
    // Allocate list of parts.  The parts are not initialized with any geometry

    pub fn allocateMemory(&mut self, nPartCount: usize) {
        // First, whack anything already allocated

        self.freeMemory();

        // Check if not allocating anything

        if nPartCount < 1 {
            return;
        }

        // Allocate lists

        self.partMeshList.reserve(nPartCount);
        self.partTextureList.reserve(nPartCount);

        self.partCount = nPartCount;
    }

    //---------------------------------------------------------------------------
    // freeMemory
    //
    // Free any resources and reset variables to empty state.

    pub fn freeMemory(&mut self) {
        self.partMeshList.clear();
        self.partTextureList.clear();
        self.partCount = 0;
    }

    //---------------------------------------------------------------------------
    // getPartMesh
    //
    // Accessor - return pointer to the part mesh, by index.
    pub fn getPartMesh(&mut self, index: usize) -> &mut TriMesh {
        &mut self.partMeshList[index]
    }

    //---------------------------------------------------------------------------
    // getPartTexture
    //
    // Accessor - return pointer to the texture reference
    pub fn getPartTexture(&mut self, index: usize) -> &mut TextureReference {
        &mut self.partTextureList[index]
    }

    //---------------------------------------------------------------------------
    // setPartTextureName
    //
    // Accessor - set texture name for a part
    pub fn setPartTextureName(&mut self, index: usize, name: &str) {
        self.partTextureList[index].name = String::from(name);
    }

    //---------------------------------------------------------------------------
    // cache
    //
    // Cache textures.  For best performance, always cache your textures
    // before rendering

    pub fn cache(&mut self, config: &mut Config) {
        // Cache all textures
        for i in 0..self.partTextureList.len() {
            config.renderer.cacheTexture(&self.partTextureList[i]);
        }
    }

    //---------------------------------------------------------------------------
    // render
    //
    // Render the parts of the model using the current 3D context.
    pub fn render(&mut self, config: &mut Config) {
        // Render all the parts
        for i in 0..self.partCount {
            self.renderPart(config, i);
        }
    }

    //---------------------------------------------------------------------------
    // renderPart
    //
    // Render a single part of the model using the current 3D context.

    pub fn renderPart(&mut self, config: &mut Config, index: usize) {
        // Sanity check
        // assert(index >= 0);
        // assert(index < partCount);
        // assert(partMeshList != NULL);
        // assert(partTextureList != NULL);

        // Select the texture

        config.renderer.selectTexture(&self.partTextureList[index]);

        // Render the part

        self.partMeshList[index].render(config);
    }

    //---------------------------------------------------------------------------
    // fromEditMesh
    //
    // Convert an EditTriMesh to a Model.  Note that this function may need
    // to make many logical changes to the mesh, such as number of actual
    // parts, ordering of vertices, materials, faces, etc.  Faces may need
    // to be detached across part boundaries.  Vertices may need to be duplictaed
    // to place UV's at the vertex level.  However, the actual mesh geometry will
    // not be modified as far as number of faces, vertex positions,
    // vertex normals, etc.
    //
    // The input mesh is not modified, except for possibly the mark fields.

    pub fn fromEditMesh(&mut self, mesh: &mut EditTriMesh) {
        // Free up anything already allocated

        self.freeMemory();

        // Make sure something exists in the destination mesh

        if mesh.partCount() < 1 {
            return;
        }

        // Extract the part meshes

        let mut partMeshes: Vec<EditTriMesh> = Vec::with_capacity(mesh.pList.len());
        mesh.extractParts(&mut partMeshes);

        // Figure out how many parts we'll need.  Remember,
        // each of our parts must have a single material,
        // so we must duplicate parts for multiple materials.

        let mut numParts = 0;
        for i in 0..mesh.pList.len() {
            numParts += partMeshes[i].materialCount();
        }

        // Allocate
        self.allocateMemory(numParts);

        // Convert each part

        let mut destPartIndex = 0;

        for i in 0..mesh.pList.len() {
            let mut srcMesh = &mut partMeshes[i];
            for j in 0..srcMesh.mList.len() {
                // Get a mesh consisting of the faces
                // in this part that use this material

                let mut onePartOneMaterial = EditTriMesh::default();
                srcMesh.extractOnePartOneMaterial(0, j, &mut onePartOneMaterial);

                // Sanity check the output mesh

                assert!(onePartOneMaterial.vertexCount() > 0);
                assert!(onePartOneMaterial.triCount() > 0);
                assert!(onePartOneMaterial.pList.len() == 1);
                assert!(onePartOneMaterial.materialCount() == 1);

                // Convert the mesh to a trimesh

                self.getPartMesh(destPartIndex)
                    .fromEditMesh(&onePartOneMaterial);

                // Convert the material

                self.setPartTextureName(
                    destPartIndex,
                    &*onePartOneMaterial.mList[0].diffuseTextureName,
                );

                // !FIXME! Need to implement part names!

                // Next destination part, please

                destPartIndex += 1;
            }
        }
        assert_eq!(destPartIndex, self.partCount);
    }

    //---------------------------------------------------------------------------
    // toEditMesh
    //
    // Convert a Model to an EditTriMesh

    pub fn toEditMesh(&mut self, mesh: &EditTriMesh) {
        // !FIXME!
        assert!(false);
    }

    //---------------------------------------------------------------------------
    // toEditMesh
    //
    // Convert a Model to an EditTriMesh

    pub fn importS3d(&mut self, s3dFilename: &str) {
        // Load up the S3D into an EditTriMesh
        let result = import_s3d(s3dFilename);

        match result {
            Ok(mut editMesh) => {
                // Optimize it for rendering
                editMesh.optimizeForRendering();
                // Convert it to renderable Model format
                self.fromEditMesh(&mut editMesh);
            }
            Err(error) => {
                panic!("{}", error);
            }
        }
    }
}

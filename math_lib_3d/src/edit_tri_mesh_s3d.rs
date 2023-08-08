use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};
use scanf::{scanf, sscanf};

use crate::edit_tri_mesh::*;

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
// import_s3d
//
// Load up an S3D file.  Returns true on success.  If failure, returns
// false and puts an error message into returnErrMsg

pub fn import_s3d(filename: &str) -> Result<EditTriMesh, Error> {
    let mut edit_mesh = EditTriMesh::default();

    // Try to open up the file
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);

    let mut lines = buffered.lines();

    if let Some(Ok(version_msg)) = lines.next() {
        if version_msg != "// version" {
            return Err(Error::new(
                ErrorKind::Other,
                "Expected version message",
            ));
        }
        if let Some(Ok(version_num)) = lines.next() {
            println!("version num: {}", version_num);
            if version_num != "103" {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("File is version {} - only version 103 supported", version_num),
                ));

            }
        }
    }

    // numTextures,numTris,numVerts,numParts,numFrames,numLights,numCameras
    if let Some(Ok(num_things)) = lines.next() {
        println!("{}", num_things);
    }

    let mut	numTextures: usize = 0;
    let mut numTris: usize = 0;
    let mut numVerts: usize = 0;
    let mut numParts: usize = 0;
    let mut numFrames: usize = 0;
    let mut numLights: usize = 0;
    let mut numCameras: usize = 0;

    if let Some(Ok(num_things)) = lines.next() {
        sscanf!(&num_things, "{},{},{},{},{},{},{}", numTextures,numTris,numVerts,numParts,numFrames,numLights,numCameras);
        println!("{},{},{},{},{},{},{}", numTextures,numTris,numVerts,numParts,numFrames,numLights,numCameras);
    }

    edit_mesh.mList = Vec::with_capacity(numTextures);
    edit_mesh.tList = Vec::with_capacity(numTris);
    edit_mesh.vList = Vec::with_capacity(numVerts);
    edit_mesh.pList = Vec::with_capacity(numParts);




    Ok(edit_mesh)
}

/*


// Read and check version

if ( ! skipLine(f)) {
corrupt:
strcpy(returnErrMsg, "File is corrupt");
goto failed;
}
int    version;
if (fscanf(f, "%d\n", & version) != 1) {
goto corrupt;
}
if (version != 103) {
sprintf(returnErrMsg, "File is version %d - only version 103 supported", version);
goto failed;
}

// Read header

if ( ! skipLine(f)) {
goto corrupt;
}
int    numTextures, numTris, numVerts, numParts, numFrames, numLights, numCameras;
if (fscanf(f, "%d , %d , %d , %d , %d , %d , %d\n", & numTextures, & numTris, &numVerts, & numParts, & numFrames, & numLights, & numCameras) != 7) {
goto corrupt;
}

// Allocate lists

setMaterialCount(numTextures);
setTriCount(numTris);
setVertexCount(numVerts);
setPartCount(numParts);

// Read part list.  the only number we care about
// is the triangle count, which we'll temporarily
// stash into the mark field

if ( ! skipLine(f)) {
goto corrupt;
}
int    firstVert = 0, firstTri = 0;
for (i = 0; i < numParts; ++ i) {
Part * p = & part(i);
int    partFirstVert, partNumVerts, partFirstTri, partNumTris;
if (fscanf(f, "%d , %d , %d , %d , \"%[^\"]\"\n", & partFirstVert, & partNumVerts, & partFirstTri, & partNumTris, p-> name) != 5) {
sprintf(returnErrMsg, "Corrupt at part %d", i);
goto failed;
}
if (firstVert != partFirstVert | | firstTri != partFirstTri) {
sprintf(returnErrMsg, "Part vertex/tri mismatch detected at part %d", i);
goto failed;
}
p -> mark = partNumTris;
firstVert += partNumVerts;
firstTri += partNumTris;
}
if (firstVert != numVerts | | firstTri != numTris) {
strcpy(returnErrMsg, "Part vertex/tri mismatch detected at end of part list");
goto failed;
}

// Read textures.

if ( ! skipLine(f)) {
goto corrupt;
}
for (i = 0; i < numTextures; + + i) {
Material * m = & material(i);

// Fetch line of text

if (fgets(m -> diffuseTextureName, sizeof(m -> diffuseTextureName), f) != m ->diffuseTextureName) {
sprintf(returnErrMsg, "Corrupt reading texture %d", i);
goto failed;
}

// Styrip off newline, which fgets leaves.
// Wouldn't it have been nice if the stdio
// functions would just have a function to read a line
// WITHOUT the newline character.  What a pain...

char * nl = strchr(m -> diffuseTextureName, '\n');
if (nl != NULL) {
* nl = '\0';
}
}

// Read triangles a part at a time

if ( ! skipLine(f)) {
goto corrupt;
}
int    whiteTextureIndex = - 1;
int    destTriIndex = 0;
for (int partIndex = 0; partIndex < numParts; + + partIndex) {

// Read all triangles in this part

for (int i = 0; i < part(partIndex).mark; + + i) {

// get shortcut to destination triangle

Tri * t = & tri(destTriIndex);

// Slam part number

t -> part = partIndex;

// Parse values from file

if (fscanf(f, "%d , %d , %f , %f , %d , %f , %f , %d , %f , %f\n",
& t -> material,
&t -> v[0].index, &t -> v[0].u, &t -> v[0].v,
&t -> v[1].index, &t -> v[1].u, &t -> v[1].v,
&t -> v[2].index, &t -> v[2].u, &t -> v[2].v
) != 10) {
sprintf(returnErrMsg, "Corrupt reading triangle %d (%d of part %d)", destTriIndex, i, partIndex);
goto failed;
}

// Check for untextured triangle

if (t -> material < 0) {
if (whiteTextureIndex < 0) {
Material whiteMaterial;
strcpy(whiteMaterial.diffuseTextureName, "White");
whiteTextureIndex = addMaterial(whiteMaterial);
}
t -> material = whiteTextureIndex;
}

// Scale UV's to 0...1 range

t -> v[0].u /= 256.0f;
t -> v[0].v /= 256.0f;
t -> v[1].u /= 256.0f;
t-> v[1].v /= 256.0f;
t -> v[2].u /= 256.0f;
t -> v[2].v /= 256.0f;

// Next triangle, please

+ +destTriIndex;
}
}
assert(destTriIndex == triCount());

// Read vertices

if ( ! skipLine(f)) {
goto corrupt;
}
for (i = 0; i < numVerts; + + i) {
Vertex * v = & vertex(i);
if (fscanf(f, "%f , %f , %f\n", & v-> p.x, & v -> p.y, & v -> p.z) != 3) {
sprintf(returnErrMsg, "Corrupt reading vertex %d", i);
goto failed;
}
}

// OK, we don't need anything from the rest of the file.  Close file.

fclose(f);
f = NULL;

// Check for structural errors in the mesh

if ( ! validityCheck(returnErrMsg)) {
goto failed;
}

// OK!

return true;
}

pub fn exportS3d(char *filename) {}
}
}
*/

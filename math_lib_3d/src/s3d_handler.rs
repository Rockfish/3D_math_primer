use debug_print::debug_println;
use scanf::sscanf;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error, ErrorKind};

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

    // Open file
    let file = File::open(filename)?;
    let buffered = BufReader::new(file);

    let mut lines = buffered.lines();

    if let Some(Ok(version_msg)) = lines.next() {
        if version_msg != "// version" {
            return Err(Error::new(ErrorKind::Other, "Expected version message"));
        }
        if let Some(Ok(version_num)) = lines.next() {
            debug_println!("version num: {}", version_num);
            if version_num != "103" {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!(
                        "File is version {} - only version 103 supported",
                        version_num
                    ),
                ));
            }
        }
    }

    // numTextures,numTris,numVerts,numParts,numFrames,numLight s,numCameras
    if let Some(Ok(num_things)) = lines.next() {
        debug_println!("{}", num_things);
    }

    let mut numTextures: usize = 0;
    let mut numTris: usize = 0;
    let mut numVerts: usize = 0;
    let mut numParts: usize = 0;
    let mut numFrames: usize = 0;
    let mut numLights: usize = 0;
    let mut numCameras: usize = 0;

    if let Some(Ok(num_things)) = lines.next() {
        sscanf!(
            &num_things,
            "{},{},{},{},{},{},{}",
            numTextures,
            numTris,
            numVerts,
            numParts,
            numFrames,
            numLights,
            numCameras
        );
        debug_println!(
            "{},{},{},{},{},{},{}",
            numTextures,
            numTris,
            numVerts,
            numParts,
            numFrames,
            numLights,
            numCameras
        );
    }

    edit_mesh.mList = Vec::with_capacity(numTextures);
    edit_mesh.tList = Vec::with_capacity(numTris);
    edit_mesh.vList = Vec::with_capacity(numVerts);
    edit_mesh.pList = Vec::with_capacity(numParts);

    // Read part list.  the only number we care about
    // is the triangle count, which we'll temporarily
    // stash into the mark field

    // skip line: partList: firstVert,numVerts,firstTri,numTris,"name"
    if let Some(Ok(num_things)) = lines.next() {
        debug_println!("{}", num_things);
    }

    let mut firstVert = 0;
    let mut firstTri = 0;

    for i in 0..numParts {
        let mut partFirstVert: usize = 0;
        let mut partNumVerts: usize = 0;
        let mut partFirstTri: usize = 0;
        let mut partNumTris: usize = 0;
        let mut name: String = String::new();

        let mut p = Part::default();

        if let Some(Ok(parts_list)) = lines.next() {
            sscanf!(
                &parts_list,
                "{},{},{},{},\"{}\"",
                partFirstVert,
                partNumVerts,
                partFirstTri,
                partNumTris,
                name
            );
            debug_println!(
                "{},{},{},{},\"{}\"",
                partFirstVert,
                partNumVerts,
                &partFirstTri,
                partNumTris,
                name
            );
        }

        if firstVert != partFirstVert || firstTri != partFirstTri {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Part vertex/tri mismatch detected at part {}", i),
            ));
        }

        p.name = name;
        p.mark = partNumTris as i32;

        firstVert += partNumVerts;
        firstTri += partNumTris;

        edit_mesh.pList.push(p);
    }

    if firstVert != numVerts || firstTri != numTris {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Part vertex/tri mismatch detected at end of part list"),
        ));
    }

    // Read textures.

    // skip line: texture list: name
    if let Some(Ok(skip_line)) = lines.next() {
        debug_println!("{}", skip_line);
    }

    for i in 0..numTextures {
        let mut m = Material::default();
        let mut name = String::new();

        if let Some(Ok(texture_name)) = lines.next() {
            sscanf!(&texture_name, "{}", name);
            debug_println!("{}", name);
        }

        m.diffuseTextureName = name;
        edit_mesh.mList.push(m);
    }

    // Read triangles a part at a time

    // skip line: triList: materialIndex,vertices(index, texX, texY)
    if let Some(Ok(skip_line)) = lines.next() {
        debug_println!("{}", skip_line);
    }

    let mut whiteTextureIndex = usize::MAX;
    let mut destTriIndex = 0;

    let mut materialIndex: i32 = 0;
    let mut v1_index: usize = 0;
    let mut v1_u: f32 = 0.0;
    let mut v1_v: f32 = 0.0;
    let mut v2_index: usize = 0;
    let mut v2_u: f32 = 0.0;
    let mut v2_v: f32 = 0.0;
    let mut v3_index: usize = 0;
    let mut v3_u: f32 = 0.0;
    let mut v3_v: f32 = 0.0;

    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    let mut z: f32 = 0.0;

    for partIndex in 0..numParts {
        // Read all triangles in this part
        let p = &edit_mesh.pList[partIndex];

        for i in 0..p.mark {
            let mut t = Tri::default();
            // Set part number
            t.part = partIndex;

            if let Some(Ok(tri_list)) = lines.next() {
                sscanf!(
                    &tri_list,
                    "{}, {},{},{}, {},{},{}, {},{},{}",
                    materialIndex,
                    v1_index,
                    v1_u,
                    v1_v,
                    v2_index,
                    v2_u,
                    v2_v,
                    v3_index,
                    v3_u,
                    v3_v,
                );

                t.material = if materialIndex < 0 {
                    usize::MAX
                } else {
                    materialIndex as usize
                };
                t.v[0].index = v1_index;
                t.v[0].u = v1_u;
                t.v[0].v = v1_v;
                t.v[1].index = v2_index;
                t.v[1].u = v2_u;
                t.v[1].v = v2_v;
                t.v[2].index = v3_index;
                t.v[2].u = v3_u;
                t.v[2].v = v3_v;

                debug_println!(
                    "{}, {},{},{}, {},{},{}, {},{},{}",
                    t.material,
                    t.v[0].index,
                    t.v[0].u,
                    t.v[0].v,
                    t.v[1].index,
                    t.v[1].u,
                    t.v[1].v,
                    t.v[2].index,
                    t.v[2].u,
                    t.v[2].v
                );
            }

            // Check for untextured triangle
            if t.material == usize::MAX {
                if whiteTextureIndex == usize::MAX {
                    let mut whiteMaterial = Material::default();
                    whiteMaterial.diffuseTextureName = String::from("White");
                    edit_mesh.mList.push(whiteMaterial);
                    whiteTextureIndex = edit_mesh.mList.len() - 1;
                }
                t.material = whiteTextureIndex;
            }

            // Scale UV's to 0...1 range
            t.v[0].u /= 256.0;
            t.v[0].v /= 256.0;
            t.v[1].u /= 256.0;
            t.v[1].v /= 256.0;
            t.v[2].u /= 256.0;
            t.v[2].v /= 256.0;

            edit_mesh.tList.push(t);
            destTriIndex += 1;
        }
    }
    assert_eq!(
        destTriIndex,
        edit_mesh.tList.len(),
        "found num of triangles doesn't match declared num of triangles"
    );

    // skip line: vertList: x,y,z
    if let Some(Ok(skip_line)) = lines.next() {
        debug_println!("{}", skip_line);
    }

    for i in 0..numVerts {
        if let Some(Ok(vertex)) = lines.next() {
            sscanf!(&vertex, "{}, {}, {}", x, y, z);
            debug_println!("{}, {}, {}", x, y, z);

            let mut v = Vertex::default();
            v.p.x = x;
            v.p.y = y;
            v.p.z = z;

            edit_mesh.vList.push(v);
        }
    }

    Ok(edit_mesh)
}

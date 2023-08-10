#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use math_lib_3d::*;
use std::f64::consts::*;
use math_lib_3d::aabb3::AABB3;

use math_lib_3d::config::Config;
use math_lib_3d::renderer::{RenderTri, RenderVertex};


/////////////////////////////////////////////////////////////////////////////
//
// 3D Math Primer for Games and Graphics Development
//
// Viewer.cpp - Test application to view a 3D model
//
// Visit gamemath.com for the latest version of this file.
//
/////////////////////////////////////////////////////////////////////////////

pub fn init() {
    /*
    // Create the main application window

    createAppWindow("Model viewer");
    // Find the most appropriate video mode to use

    //int	modeIndex = -1;
    //for (i = 0 ; i < gRenderer.getVideoModeCount() ; ++i) {
    //	VideoMode mode = gRenderer.getVideoMode(i);
    //	if (
    //		(mode.xRes == 800) &&
    //		(mode.yRes == 600) &&
    //		((mode.bitsPerPixel == 32) || (mode.bitsPerPixel == 24))
    //	) {
    //		modeIndex = i;
    //		break;
    //	}
    //}
    //if (modeIndex < 0) {
    //	ABORT("Can't set video mode");
    //}
    //VideoMode mode = gRenderer.getVideoMode(modeIndex)

        let mode: VideoMode = VideoMode::default();
    mode.xRes = 800;
    mode.yRes = 600;
    mode.bitsPerPixel = 24;
    mode.refreshHz = kRefreshRateDefault;

    // Set the mode

    gRenderer.init(mode);

     */
}

pub fn shutdown(config: &Config) {
    //config.renderer.shutdown();
    //destroyAppWindow();
}

pub fn renderCube(config: &Config) {
    let mut cube: AABB3 = AABB3::new();

    cube.min.x = -5.0;
    cube.min.y = -5.0;
    cube.min.z = -5.0;
    cube.max = -cube.min;

    let vl: Vec<RenderVertex> = vec![];

    for i in 0..8 {
        let mut rv: RenderVertex = RenderVertex::default();

        rv.p = cube.corner(i);
//vl[i].argb = MAKE_ARGB(255, (i & 1) ? 255 : 0, (i & 2) ? 255 : 0, (i & 4) ? 255 : 0);
        rv.n = rv.p;
        rv.n.normalize();
        rv.u = if i & 1 { 1.0 } else { 0.0 };
        rv.v = if i & 2 { 1.0 } else { 0.0 };
    }

    let mut pl: Vec<RenderTri> = vec![];

    pl.push(RenderTri::new(0, 4, 6));
    pl.push(RenderTri::new(0, 4, 6));
    pl.push(RenderTri::new(0, 6, 2));
    pl.push(RenderTri::new(1, 3, 7));
    pl.push(RenderTri::new(1, 7, 5));
    pl.push(RenderTri::new(0, 1, 5));
    pl.push(RenderTri::new(0, 5, 4));
    pl.push(RenderTri::new(2, 6, 7));
    pl.push(RenderTri::new(2, 7, 3));
    pl.push(RenderTri::new(0, 2, 3));
    pl.push(RenderTri::new(0, 3, 1));
    pl.push(RenderTri::new(4, 5, 7));
    pl.push(RenderTri::new(4, 7, 6));

    config.renderer.renderTriMesh(vl, 8, pl, 12);
}


fn main() {
    /*

// Setup program

init();

// Set the window

gRenderer.setFullScreenWindow();

// Set the camera a little bit south and above
// the origin, looking slightly down and to the north

EulerAngles cameraOrient;
cameraOrient.heading = 0.0f;
cameraOrient.pitch = degToRad(30.0f);
cameraOrient.bank = 0.0f;
gRenderer.setCamera(Vector3(0.0f, 20.0f, -40.0f), cameraOrient);
gRenderer.setZoom(fovToZoom(degToRad(60.0f)));

// Load model

Model model;
model.importS3d("ar_couch.s3d");
model.cache();

// Spin a cube

EulerAngles orient = kEulerAnglesIdentity;
while (!gQuitFlag) {

// Get ready to draw

gRenderer.beginScene();
gRenderer.clear();

// Render a cube

gRenderer.setLightEnable(true);
gRenderer.instance(kZeroVector, orient);
//renderCube();
model.render();
gRenderer.instancePop();

// Show it

gRenderer.endScene();
gRenderer.flipPages();

// Rotate cube's heading

orient.heading += .01f;

// Check for ESC to exit the app

if (gKeyboard.debounce(kKeyEsc)) {
break;
}

}

// Shutdown

shutdown();
    */
}

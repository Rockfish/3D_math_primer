#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::renderer::make_argb;
use crate::utils::{read_raw_struct, read_u8};
use debug_print::debug_print;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug)]
pub enum EFormat {
    eFormat_None, // dummy placeholder value
    eFormat_8888, // 32-bit ARGB
                  // !KLUDGE! FOr now, this is all we'll support.
}

#[derive(Debug)]
pub struct Bitmap {
    pub sizeX: usize,
    pub sizeY: usize,
    pub fmt: EFormat,
    pub data: Vec<u32>,
}

#[derive(Debug)]
#[repr(packed)]
pub struct TGAHeader {
    pub imageIDLength: u8,
    pub colorMapType: u8,
    pub imageType: u8,
    pub colorMapFirstIndex: u16,
    pub colorMapLength: u16,
    pub colorMapBitsPerEntry: u8,
    pub xOrigin: u16,
    pub yOrigin: u16,
    pub width: u16,
    pub height: u16,
    pub bitsPerPixel: u8,
    pub imageDescriptor: u8,
}

impl Bitmap {
    pub fn default() -> Bitmap {
        Bitmap {
            sizeX: 0,
            sizeY: 0,
            fmt: EFormat::eFormat_None,
            data: vec![],
        }
    }

    pub fn allocateMemory(&mut self, xs: usize, ys: usize, format: EFormat) {
        assert!(xs > 0 && ys > 0);

        let mut rowBytes: usize = 0;

        match format {
            EFormat::eFormat_8888 => {
                rowBytes = xs * 4;
            }
            _ => {
                assert!(false, "unsupported file format")
            }
        }

        self.data = Vec::with_capacity(rowBytes);
        self.sizeX = xs;
        self.sizeY = ys;
        self.fmt = format;
    }

    pub fn freeMemory(&mut self) {
        self.data.clear();
        self.sizeX = 0;
        self.sizeY = 0;
        self.fmt = EFormat::eFormat_None;
    }

    //---------------------------------------------------------------------------
    // pub fn getPix
    //
    // Fetch a pixel at the given coordinates.  The pixel is always returned in
    // 32-bit 0xAARRGGBB format, the same as used by the Renderer class
    // and MAKE_ARGB macro.

    pub fn getPix(&self, x: usize, y: usize) -> u32 {
        // Safety check

        if (x >= self.sizeX) || (y >= self.sizeY) || (self.data.is_empty()) {
            assert!(false, "coordinates out of bounds");
            return 0;
        }

        // Check format

        let mut result: u32 = 0;
        match &self.fmt {
            EFormat::eFormat_8888 => {
                result = self.data[y * self.sizeX + x];
            }
            _ => {
                assert!(false);
                result = 0;
            }
        }

        // Return it
        result
    }

    //---------------------------------------------------------------------------
    // pub fn setPix
    //
    // Set a pixel at the given coordinates.  The pixel is specified in
    // 32-bit 0xAARRGGBB format, the same as used by the Renderer class
    // and MAKE_ARGB macro.

    pub fn setPix(&mut self, x: usize, y: usize, argb: u32) {
        // Safety check

        if (x >= self.sizeX) || (y >= self.sizeY) || (self.data.is_empty()) {
            assert!(false, "coordinates out of bounds");
            return;
        }

        // Check format

        match &self.fmt {
            EFormat::eFormat_8888 => {
                self.data[y * self.sizeX + x] = argb;
            }
            _ => {
                assert!(false);
            }
        }
    }

    //---------------------------------------------------------------------------
    // pub fn load
    //
    // Load a bitmap from an image file.  Uses the extension to
    // figure out how to load.

    pub fn load(&mut self, filename: &str) -> Result<bool, String> {
        // Free up anything already allocated

        self.freeMemory();

        // Fetch extension.  I wish I could use the _splitpath function,
        // but it's not cross-platform.  I'll parse the thing myself.

        // Check for known extensions

        if filename.ends_with(".tga") {
            return self.loadTGA(filename);
        }
        if filename.ends_with(".bmp") {
            return self.loadBMP(filename);
        }

        Err("Unknown/unsupported file extension '%s'".parse().unwrap())
    }

    //---------------------------------------------------------------------------
    // pub fn loadTGA
    //
    // Load image in .TGA format.

    pub fn loadTGA(&mut self, filename: &str) -> Result<bool, String> {
        // Cleanup
        self.freeMemory();

        // Open the file
        let file = File::open(filename).unwrap();

        // Read TGA header
        let header: TGAHeader;
        let r = read_raw_struct::<File, TGAHeader>(&file);
        match r {
            Ok(data) => {
                header = data;
            }
            Err(message) => {
                debug_print!("Error: {}", message.to_string());
                return Err(String::from("I/O error, or file is corrupt."));
            }
        }

        // Check format

        if header.imageType == 2 {
            // UNCOMPRESSED_TRUECOLOR
            if (header.bitsPerPixel != 24) && (header.bitsPerPixel != 32) {
                return Err(format!(
                    "{}-bit truecolor image not supported",
                    header.bitsPerPixel
                ));
            }
            if header.colorMapType != 0 {
                return Err(String::from("Truecolor image with colormap not supported"));
            }

        //} else if (head.imageType == 1) { // UNCOMPRESSED_COLORMAPPED
        //	if (
        //		(head.colorMapType != 1) ||
        //		(head.bitsPerPixel != 8) ||
        //		(head.colorMapFirstIndex != 0) ||
        //		(head.colorMapLength != 256) ||
        //		(head.colorMapBitsPerEntry != 24)
        //	) {
        //		strcpy(returnErrMsg, "Invalid colormapped image file format");
        //		return 0;
        //	}
        } else {
            return Err(format!(
                ".TGA image type {} not supported",
                header.imageType
            ));
        }

        // Check origin

        // assert!(!(header.imageDescriptor & 0x10)); // x origin at the right not supported

        // Allocate image of the correct size

        self.allocateMemory(
            header.width as usize,
            header.height as usize,
            EFormat::eFormat_8888,
        );

        // Read the image data, in file order

        let mut buffered = BufReader::new(file);

        //let rowSz = header.bitsPerPixel / 8 * (self.sizeX as u8);
        for y in 0..self.sizeY {
            // Figure out which row this is in the image.
            // TGA's can be stored "upside down"

            let dy;
            if (header.imageDescriptor & 0x20) == 0x20 {
                dy = y;
            } else {
                dy = self.sizeY - y - 1;
            }

            // Read in the data for this row

            for _x in 0..self.sizeX {
                let b = read_u8(&mut buffered);
                let g = read_u8(&mut buffered);
                let r = read_u8(&mut buffered);

                let a = if header.bitsPerPixel == 24 {
                    255
                } else {
                    read_u8(&mut buffered)
                };

                // assert!(!(b < 0 || g < 0 || r < 0 || a < 0), "bad values");

                let argb = make_argb(a as u32, r as u32, g as u32, b as u32);

                self.data.push(argb);
            }
        }
        Ok(true)
    }

    //---------------------------------------------------------------------------
    // pub fn loadBMP
    //
    // Load image in .BMP format.

    pub fn loadBMP(&mut self, _filename: &str) -> Result<bool, String> {
        // Free up anything already allocated
        self.freeMemory();
        todo!();
    }
}

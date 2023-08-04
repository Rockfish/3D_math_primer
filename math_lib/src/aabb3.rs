#![allow(dead_code)]

use std::ops::Mul;
// use std::ops::Mul;
use crate::matrix4x3::*;
use crate::vector::*;

// Implement a 3D axially aligned bounding box

#[derive(Clone, Debug)]
struct AABB3 {
    min: Vec3,
    max: Vec3,
}

impl AABB3 {
    // Query for dimensions

    pub fn size(&self) -> Vec3 {
        &self.max - &self.min
    }

    pub fn x_size(&self) -> f32 {
        self.max.x - self.min.x
    }

    pub fn y_size(&self) -> f32 {
        self.max.y - self.min.y
    }

    pub fn z_size(&self) -> f32 {
        self.max.z - self.min.z
    }

    pub fn center(&self) -> Vec3 {
        (&self.min + &self.max).mul(0.5)
    }

    //---------------------------------------------------------------------------
    // corner
    //
    // Return one of the 8 corner points.  The points are numbered as follows:
    //
    //            6                                7
    //              ------------------------------
    //             /|                           /|
    //            / |                          / |
    //           /  |                         /  |
    //          /   |                        /   |
    //         /    |                       /    |
    //        /     |                      /     |
    //       /      |                     /      |
    //      /       |                    /       |
    //     /        |                   /        |
    //  2 /         |                3 /         |
    //   /----------------------------/          |
    //   |          |                 |          |
    //   |          |                 |          |      +Y
    //   |        4 |                 |          |
    //   |          |-----------------|----------|      |
    //   |         /                  |         /  5    |
    //   |        /                   |        /        |       +Z
    //   |       /                    |       /         |
    //   |      /                     |      /          |     /
    //   |     /                      |     /           |    /
    //   |    /                       |    /            |   /
    //   |   /                        |   /             |  /
    //   |  /                         |  /              | /
    //   | /                          | /               |/
    //   |/                           |/                ----------------- +X
    //   ------------------------------
    //  0                              1
    //
    // Bit 0 selects min.x vs. max.x
    // Bit 1 selects min.y vs. max.y
    // Bit 2 selects min.z vs. max.z

    pub fn corner(&self, i: i32) -> Vec3 {
        // Make sure index is in range...
        assert!(i >= 0);
        assert!(i <= 7);
        Vec3 {
            x: if (i & 1) == 1 { self.max.x } else { self.min.x },
            y: if (i & 2) == 2 { self.max.y } else { self.min.y },
            z: if (i & 4) == 4 { self.max.z } else { self.min.z },
        }
    }

    // "Empty" the box, by setting the values to really
    // large/small numbers
    pub fn empty(&mut self) {
        let k_big_number = f32::MAX;
        self.min.x = k_big_number;
        self.min.y = k_big_number;
        self.min.z = k_big_number;

        self.max.x = -k_big_number;
        self.max.y = -k_big_number;
        self.max.z = -k_big_number;
    }

    // Add a point to the box
    // Expand the box as necessary to contain the point.
    pub fn add_point(&mut self, p: &Vec3) {
        if p.x < self.min.x {
            self.min.x = p.x
        };
        if p.x > self.max.x {
            self.max.x = p.x
        };
        if p.y < self.min.x {
            self.min.y = p.y
        };
        if p.y > self.max.x {
            self.max.y = p.y
        };
        if p.z < self.min.x {
            self.min.z = p.z
        };
        if p.z > self.max.x {
            self.max.z = p.z
        };
    }

    // Add an AABB to the box
    pub fn add_aabb(&mut self, box_aabb3: &AABB3) {
        // Expand the box as necessary.

        if box_aabb3.min.x < self.min.x {
            self.min.x = box_aabb3.min.x
        };
        if box_aabb3.min.x > self.max.x {
            self.max.x = box_aabb3.min.x
        };
        if box_aabb3.min.y < self.min.x {
            self.min.y = box_aabb3.min.y
        };
        if box_aabb3.min.y > self.max.x {
            self.max.y = box_aabb3.min.y
        };
        if box_aabb3.min.z < self.min.x {
            self.min.z = box_aabb3.min.z
        };
        if box_aabb3.min.z > self.max.x {
            self.max.z = box_aabb3.min.z
        };
    }

    //---------------------------------------------------------------------------
    // set_to_transformed_box
    // Transform the box and compute the new AABB.  Remember, this always
    // results in an AABB that is at least as big as the origin, and may be
    // considerably bigger.
    pub fn set_to_transformed_box(&mut self, box_aabb3: &AABB3, m: &Matrix4x3) {
        // If we're empty, then bail

        if box_aabb3.is_empty() {
            self.empty();
            return;
        }

        // Start with the translation portion

        self.min = get_translation(m);
        self.max = get_translation(m);

        // Examine each of the 9 matrix elements
        // and compute the new AABB

        if m.m11 > 0.0 {
            self.min.x += m.m11 * box_aabb3.min.x;
            self.max.x += m.m11 * box_aabb3.max.x;
        } else {
            self.min.x += m.m11 * box_aabb3.max.x;
            self.max.x += m.m11 * box_aabb3.min.x;
        }

        if m.m12 > 0.0 {
            self.min.y += m.m12 * box_aabb3.min.x;
            self.max.y += m.m12 * box_aabb3.max.x;
        } else {
            self.min.y += m.m12 * box_aabb3.max.x;
            self.max.y += m.m12 * box_aabb3.min.x;
        }

        if m.m13 > 0.0 {
            self.min.z += m.m13 * box_aabb3.min.x;
            self.max.z += m.m13 * box_aabb3.max.x;
        } else {
            self.min.z += m.m13 * box_aabb3.max.x;
            self.max.z += m.m13 * box_aabb3.min.x;
        }

        if m.m21 > 0.0 {
            self.min.x += m.m21 * box_aabb3.min.y;
            self.max.x += m.m21 * box_aabb3.max.y;
        } else {
            self.min.x += m.m21 * box_aabb3.max.y;
            self.max.x += m.m21 * box_aabb3.min.y;
        }

        if m.m22 > 0.0 {
            self.min.y += m.m22 * box_aabb3.min.y;
            self.max.y += m.m22 * box_aabb3.max.y;
        } else {
            self.min.y += m.m22 * box_aabb3.max.y;
            self.max.y += m.m22 * box_aabb3.min.y;
        }

        if m.m23 > 0.0 {
            self.min.z += m.m23 * box_aabb3.min.y;
            self.max.z += m.m23 * box_aabb3.max.y;
        } else {
            self.min.z += m.m23 * box_aabb3.max.y;
            self.max.z += m.m23 * box_aabb3.min.y;
        }

        if m.m31 > 0.0 {
            self.min.x += m.m31 * box_aabb3.min.z;
            self.max.x += m.m31 * box_aabb3.max.z;
        } else {
            self.min.x += m.m31 * box_aabb3.max.z;
            self.max.x += m.m31 * box_aabb3.min.z;
        }

        if m.m32 > 0.0 {
            self.min.y += m.m32 * box_aabb3.min.z;
            self.max.y += m.m32 * box_aabb3.max.z;
        } else {
            self.min.y += m.m32 * box_aabb3.max.z;
            self.max.y += m.m32 * box_aabb3.min.z;
        }

        if m.m33 > 0.0 {
            self.min.z += m.m33 * box_aabb3.min.z;
            self.max.z += m.m33 * box_aabb3.max.z;
        } else {
            self.min.z += m.m33 * box_aabb3.max.z;
            self.max.z += m.m33 * box_aabb3.min.z;
        }
    }

    // Return true if the box is empty
    pub fn is_empty(&self) -> bool {
        // Check if we're inverted on any axis
        (self.min.x > self.max.x) || (self.min.y > self.max.y) || (self.min.z > self.max.z)
    }

    // contains
    // Return true if the box contains a point
    pub fn contains(&self, p: &Vec3) -> bool {
        // Check for overlap on each axis
        (p.x >= self.min.x)
            && (p.x <= self.max.x)
            && (p.y >= self.min.y)
            && (p.y <= self.max.y)
            && (p.z >= self.min.z)
            && (p.z <= self.max.z)
    }

    // Return the closest point on this box to another point
    pub fn closest_point_to(&self, p: &Vec3) -> Vec3 {
        let mut r: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        // "Push" p into the box, on each dimension
        if p.x < self.min.x {
            r.x = self.min.x;
        } else if p.x > self.max.x {
            r.x = self.max.x;
        } else {
            r.x = p.x;
        }

        if p.y < self.min.y {
            r.y = self.min.y;
        } else if p.y > self.max.y {
            r.y = self.max.y;
        } else {
            r.y = p.y;
        }

        if p.z < self.min.z {
            r.z = self.min.z;
        } else if p.z > self.max.z {
            r.z = self.max.z;
        } else {
            r.z = p.z;
        }

        r
    }

    // Return true if we intersect a sphere.  Uses Arvo's algorithm.
    pub fn intersects_sphere(&self, center: &Vec3, radius: f32) -> bool {
        // Find the closest point on box to the point

        let closest_point = self.closest_point_to(center);

        // Check if it's within range

        distance_squared(center, &closest_point) < radius * radius
    }

    // ray_intersect
    // Parametric intersection with a ray.  Returns parametric point
    // of intersection in range 0...1 or a really big number (>1) if no
    // intersection.
    //
    // From "Fast Ray-Box Intersection," by Woo in Graphics Gems I, page 395.
    pub fn ray_intersect(
        &self,
        ray_org: &Vec3,                   // origin of the ray
        ray_delta: &Vec3,                 // length and direction of the ray
        return_normal: Option<&mut Vec3>, // optionally, the normal is returned
    ) -> f32 {
        // We'll return this huge number if no intersection

        let k_no_intersection = f32::MAX;

        // Check for point inside box, trivial reject, and determine parametric
        // distance to each front face

        let mut inside = true;

        let mut xt: f32;
        let mut xn: f32 = 0.0;

        if ray_org.x < self.min.x {
            xt = self.min.x - ray_org.x;
            if xt > ray_delta.x {
                return k_no_intersection;
            }
            xt /= ray_delta.x;
            inside = false;
            xn = -1.0;
        } else if ray_org.x > self.max.x {
            xt = self.max.x - ray_org.x;
            if xt < ray_delta.x {
                return k_no_intersection;
            }
            xt /= ray_delta.x;
            inside = false;
            xn = 1.0;
        } else {
            xt = -1.0;
        }

        let mut yt: f32;
        let mut yn: f32 = 0.0;

        if ray_org.y < self.min.y {
            yt = self.min.y - ray_org.y;
            if yt > ray_delta.y {
                return k_no_intersection;
            }
            yt /= ray_delta.y;
            inside = false;
            yn = -1.0;
        } else if ray_org.y > self.max.y {
            yt = self.max.y - ray_org.y;
            if yt < ray_delta.y {
                return k_no_intersection;
            }
            yt /= ray_delta.y;
            inside = false;
            yn = 1.0;
        } else {
            yt = -1.0;
        }

        let mut zt: f32;
        let mut zn: f32 = 0.0;
        if ray_org.z < self.min.z {
            zt = self.min.z - ray_org.z;
            if zt > ray_delta.z {
                return k_no_intersection;
            }
            zt /= ray_delta.z;
            inside = false;
            zn = -1.0;
        } else if ray_org.z > self.max.z {
            zt = self.max.z - ray_org.z;
            if zt < ray_delta.z {
                return k_no_intersection;
            }
            zt /= ray_delta.z;
            inside = false;
            zn = 1.0;
        } else {
            zt = -1.0;
        }

        // Inside box?
        if inside {
            if let Some(vec) = return_normal {
                vec.x = -ray_delta.x;
                vec.y = -ray_delta.y;
                vec.z = -ray_delta.z;
                vec.normalize();
            }
            return 0.0;
        }

        // Select farthest plane - this is
        // the plane of intersection.

        let mut which = 0;
        let mut t = xt;
        if yt > t {
            which = 1;
            t = yt;
        }
        if zt > t {
            which = 2;
            t = zt;
        }

        match which {
            // intersect with yz plane
            0 =>
            {
                let y = ray_org.y + ray_delta.y * t;
                if y < self.min.y || y > self.max.y {
                    return k_no_intersection;
                }
                let z = ray_org.z + ray_delta.z * t;
                if z < self.min.z || z > self.max.z {
                    return k_no_intersection;
                }

                if let Some(vec) = return_normal {
                    vec.x = xn;
                    vec.y = 0.0;
                    vec.z = 0.0;
                }
            }
            // intersect with xz plane
            1 =>
            {
                let x = ray_org.x + ray_delta.x * t;
                if x < self.min.x || x > self.max.x {
                    return k_no_intersection;
                }
                let z = ray_org.z + ray_delta.z * t;
                if z < self.min.z || z > self.max.z {
                    return k_no_intersection;
                }

                if let Some(vec) = return_normal {
                    vec.x = 0.0;
                    vec.y = yn;
                    vec.z = 0.0;
                }
            }
            // intersect with xy plane
            2 =>
            {
                let x = ray_org.x + ray_delta.x * t;
                if x < self.min.x || x > self.max.x {
                    return k_no_intersection;
                }
                let y = ray_org.y + ray_delta.y * t;
                if y < self.min.y || y > self.max.y {
                    return k_no_intersection;
                }

                if let Some(vec) = return_normal {
                    vec.x = 0.0;
                    vec.y = 0.0;
                    vec.z = zn;
                }
            }
            _ => {}
        }

        // Return parametric point of intersection
        t
    }

    //---------------------------------------------------------------------------
    // classify_plane
    //
    // Perform static AABB-plane intersection test.  Returns:
    //
    // <0	Box is completely on the BACK side of the plane
    // >0	Box is completely on the FRONT side of the plane
    // 0	Box intersects the plane
    pub fn classify_plane(&self, n: &Vec3, d: f32) -> i32 {
        // Inspect the normal and compute the minimum and maximum
        // D values.

        let mut min_d;
        let mut max_d;

        if n.x > 0.0 {
            min_d = n.x * self.min.x;
            max_d = n.x * self.max.x;
        } else {
            min_d = n.x * self.max.x;
            max_d = n.x * self.min.x;
        }

        if n.y > 0.0 {
            min_d += n.y * self.min.y;
            max_d += n.y * self.max.y;
        } else {
            min_d += n.y * self.max.y;
            max_d += n.y * self.min.y;
        }

        if n.z > 0.0 {
            min_d += n.z * self.min.z;
            max_d += n.z * self.max.z;
        } else {
            min_d += n.z * self.max.z;
            max_d += n.z * self.min.z;
        }

        // Check if completely on the front side of the plane
        if min_d >= d {
            return 1;
        }

        // Check if completely on the back side of the plane
        if max_d <= d {
            return -1;
        }

        // We straddle the plane
        0
    }

    //---------------------------------------------------------------------------
    // intersect_plane
    //
    // Perform dynamic AABB-plane intersection test.
    //
    // n		is the plane normal (assumed to be normalized)
    // plane_d	is the D value of the plane equation p.n = d
    // dir		dir is the direction of movement of the AABB.
    //
    // The plane is assumed to be stationary.
    //
    // Returns the parametric point of intersection - the distance traveled
    // before an intersection occurs.  If no intersection, a REALLY big
    // number is returned.  You must check against the length of the
    // displacement.
    //
    // Only intersections with the front side of the plane are detected
    pub fn intersect_plane(&self, n: &Vec3, plane_d: f32, dir: &Vec3) -> f32 {
        // Make sure they are passing in normalized vectors

        assert!((n.dot(n) - 1.0).abs() < 0.01);
        assert!((dir.dot(dir) - 1.0).abs() < 0.01);

        // We'll return this huge number if no intersection

        let k_no_intersection = f32::MAX;

        // Compute glancing angle, make sure we are moving towards
        // the front of the plane

        let dot = n.dot(dir);
        if dot >= 0.0 {
            return k_no_intersection;
        }

        // Inspect the normal and compute the minimum and maximum
        // D values.  min_d is the D value of the "frontmost" corner point

        let mut min_d: f32;
        let mut max_d: f32;

        if n.x > 0.0 {
            min_d = n.x * self.min.x;
            max_d = n.x * self.max.x;
        } else {
            min_d = n.x * self.max.x;
            max_d = n.x * self.min.x;
        }

        if n.y > 0.0 {
            min_d += n.y * self.min.y;
            max_d += n.y * self.max.y;
        } else {
            min_d += n.y * self.max.y;
            max_d += n.y * self.min.y;
        }

        if n.z > 0.0 {
            min_d += n.z * self.min.z;
            max_d += n.z * self.max.z;
        } else {
            min_d += n.z * self.max.z;
            max_d += n.z * self.min.z;
        }

        // Check if we're already completely on the other
        // side of the plane

        if max_d <= plane_d {
            return k_no_intersection;
        }

        // Perform standard raytrace equation using the
        // front-most corner point

        let t = (plane_d - min_d) / dot;

        // Were we already penetrating?

        if t < 0.0 {
            return 0.0;
        }

        // Return it.  If > l, then we didn't hit in time.  That's
        // the condition that the caller should be checking for.
        t
    }

    //---------------------------------------------------------------------------
    // intersect_aabbs
    //
    // Check if two AABBs intersect, and return true if so.  Optionally return
    // the AABB of their intersection if an intersection is detected
    pub fn intersect_aabbs(box1: &AABB3, box2: &AABB3, box_intersect: Option<&mut AABB3>) -> bool {
        // Check for no overlap
        if box1.min.x > box2.max.x {
            return false;
        }
        if box1.max.x < box2.min.x {
            return false;
        }
        if box1.min.y > box2.max.y {
            return false;
        }
        if box1.max.y < box2.min.y {
            return false;
        }
        if box1.min.z > box2.max.z {
            return false;
        }
        if box1.max.z < box2.min.z {
            return false;
        }

        // We have overlap.  Compute AABB of intersection, if they want it
        if let Some(box_intersect) = box_intersect {
            box_intersect.min.x = box1.min.x.max(box2.min.x);
            box_intersect.max.x = box1.max.x.min(box2.max.x);
            box_intersect.min.y = box1.min.y.max(box2.min.y);
            box_intersect.max.y = box1.max.y.min(box2.max.y);
            box_intersect.min.z = box1.min.z.max(box2.min.z);
            box_intersect.max.z = box1.max.z.min(box2.max.z);
        }

        // They intersected
        true
    }

    //---------------------------------------------------------------------------
    // intersect_moving_aabb
    //
    // Return parametric point in time when a moving AABB collides
    // with a stationary AABB.  Returns > 1 if no intersection
    pub fn intersect_moving_aabb(stationary_box: &AABB3, moving_box: &AABB3, d: &Vec3) -> f32 {
        // We'll return this huge number if no intersection

        let k_no_intersection = f32::MAX;

        // Initialize interval to contain all the time under consideration
        let mut t_enter = 0.0;
        let mut t_leave = 1.0;

        //
        // Compute interval of overlap on each dimension, and intersect
        // this interval with the interval accumulated so far.  As soon as
        // an empty interval is detected, return a negative result
        // (no intersection.)  In each case, we have to be careful for
        // an infinite of empty interval on each dimension
        //

        // Check x-axis
        if d.x == 0.0 {
            // Empty or infinite interval on x
            if (stationary_box.min.x >= moving_box.max.x)
                || (stationary_box.max.x <= moving_box.min.x)
            {
                // Empty time interval, so no intersection
                return k_no_intersection;
            }

        // Infinite time interval - no update necessary
        } else {
            // Divide once

            let one_over_d = 1.0 / d.x;

            // Compute time value when they begin and end overlapping
            let mut x_enter = (stationary_box.min.x - moving_box.max.x) * one_over_d;
            let mut x_leave = (stationary_box.max.x - moving_box.min.x) * one_over_d;

            // Check for interval out of order
            if x_enter > x_leave {
                //swap(x_enter, x_leave);
                (x_leave, x_enter) = (x_enter, x_leave);
            }

            // Update interval
            if x_enter > t_enter {
                t_enter = x_enter;
            }
            if x_leave < t_leave {
                t_leave = x_leave;
            }

            // Check if this resulted in empty interval
            if t_enter > t_leave {
                return k_no_intersection;
            }
        }

        // Check y-axis
        if d.y == 0.0 {
            // Empty or infinite interval on y

            if (stationary_box.min.y >= moving_box.max.y)
                || (stationary_box.max.y <= moving_box.min.y)
            {
                // Empty time interval, so no intersection
                return k_no_intersection;
            }

        // Infinite time interval - no update necessary
        } else {
            // Divide once
            let one_over_d = 1.0 / d.y;

            // Compute time value when they begin and end overlapping
            let mut y_enter = (stationary_box.min.y - moving_box.max.y) * one_over_d;
            let mut y_leave = (stationary_box.max.y - moving_box.min.y) * one_over_d;

            // Check for interval out of order
            if y_enter > y_leave {
                (y_leave, y_enter) = (y_enter, y_leave);
            }

            // Update interval
            if y_enter > t_enter {
                t_enter = y_enter;
            }
            if y_leave < t_leave {
                t_leave = y_leave;
            }

            // Check if this resulted in empty interval
            if t_enter > t_leave {
                return k_no_intersection;
            }
        }

        // Check z-axis
        if d.z == 0.0 {
            // Empty or infinite interval on z
            if (stationary_box.min.z >= moving_box.max.z)
                || (stationary_box.max.z <= moving_box.min.z)
            {
                // Empty time interval, so no intersection
                return k_no_intersection;
            }

        // Infinite time interval - no update necessary
        } else {
            // Divide once
            let one_over_d = 1.0 / d.z;

            // Compute time value when they begin and end overlapping
            let mut z_enter = (stationary_box.min.z - moving_box.max.z) * one_over_d;
            let mut z_leave = (stationary_box.max.z - moving_box.min.z) * one_over_d;

            // Check for interval out of order
            if z_enter > z_leave {
                //swap(&mut z_enter, &mut z_leave);
                (z_leave, z_enter) = (z_enter, z_leave);
            }

            // Update interval
            if z_enter > t_enter {
                t_enter = z_enter;
            }
            if z_leave < t_leave {
                t_leave = z_leave;
            }

            // Check if this resulted in empty interval
            if t_enter > t_leave {
                return k_no_intersection;
            }
        }

        // OK, we have an intersection.  Return the parametric point in time
        // where the intersection occurs
        t_enter
    }
}

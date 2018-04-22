use std::f32;

/// a 2D vector with x, and y position as f32
struct vect_2D{
    x: f32,
    y: f32
}


impl vect_2D{

    pub fn add(&mut self, v:&vect_2D){
        self.x += v.x;
        self.y += v.y;
    }

    pub fn dot_product(&self, v:&vect_2D)-> f32{
        self.x * v.x + self.y * v.y
    }

    pub fn distance_from_origins(&self) -> f32{
        (self.x.powf(2.0f32) + self.y.powf(2.0f32)).sqrt()
    }

    // to recheck for rad
    pub fn rad_to_origins(&self) -> f32 {
        let mut angle: f32;

        if self.x == 0.0 && self.y == 0.0 {
            angle = 0.0;
        }

        else if self.x == 0.0 {
            angle = f32::consts::PI / 2f32;

        }

        else {
            // atan return a value comprise between [-pi/2, pi/2]
            angle = (self.y / self.x).atan();
            // first quadrant
            if self.y >= 0.0 && self.x >= 0.0{

            }
            else if self.y>=0.0 && self.x <= 0.0 {
                angle += 2f32*f32::consts::PI;

            }
            else if self.y < 0.0 && self.x< 0.0{
                angle += f32::consts::PI;
            }
            else if self.y < 0.0 && self.x >= 0.0{
                angle += 2f32 * f32::consts::PI;
            }

        }

        angle
    }

    pub fn to_angular_2D(&self) -> angular_2D{
        let rayon = self.distance_from_origins();
        let angle = self.rad_to_origins();
        angular_2D{r:rayon, theta:angle}

    }
}


///an angular vec with r as rayon and theta in radians from origins both f32.
struct angular_2D{
    r: f32,
    theta: f32
}

//impl angular_2D{
//
//
//}

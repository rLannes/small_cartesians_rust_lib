use std::f32;

/// a 2D vector with x, and y position as f32
struct Vect2D {
    x: f32,
    y: f32
}


impl Vect2D {

    pub fn add_vec(&mut self, v:&Vect2D){
        self.x += v.x;
        self.y += v.y;
    }

    pub fn dot_product(&self, v:&Vect2D) -> f32{
        self.x * v.x + self.y * v.y
    }
    /// compute distance to origines
    pub fn distance_from_origins(&self) -> f32{
        (self.x.powf(2.0f32) + self.y.powf(2.0f32)).sqrt()
    }

    // to recheck for rad
    /// compute angl fro origin in radians[0, 2pi]
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
    /// return an angular 2D that correspond to self
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

struct rectangle{
    up_left: Vect2D,
    down_right: Vect2D
}

impl rectangle{

    pub fn height(&self) -> f32{
        (self.up_left.y - self.down_right.y).abs()
    }

    pub fn width(&self) -> f32{
        (self.up_left.x - self.down_right.y).abs()
    }
    pub fn perimeter(&self)->f32{
        2f32 * self.width() + 2f32 * self.height()
    }

    pub fn area(&self) -> f32{
        self.width() * self.height()
    }

   // pub fn center(&self) -> Vect2D {
    //}
}
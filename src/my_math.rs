use bevy::math::Vec2;
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(coo1: f32, coo2:f32) -> Self {
        Self {
            x : coo1,
            y : coo2,
        }
    }
}

pub fn angle(vec1: Vector2D, vec2: Vector2D) -> f32 {
    let scalar = (vec1.x*vec2.x)+(vec1.y*vec2.y);

    let betrag1 = ((vec1.x*vec1.x) + (vec2.y*vec2.y)).sqrt(); 
    let betrag2 = ((vec2.x*vec2.x) + (vec2.y*vec2.y)).sqrt(); 

    let cos = (scalar)/(betrag1*betrag2);

    cos.acos()
}

pub fn kürze(input: Vec2) -> Vec2 {
    let mut a = input.x as i32;
    let mut b = input.y as i32;

    if a == 0 || b == 0 {
        input
    }else {
        while b != 0 {
            if a > b {
                a = a - b;
            }else {
                b = b - a;
            }
        }

        Vec2::new(input.x/a as f32, input.y/a as f32)
    }
 }





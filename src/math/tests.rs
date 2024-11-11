//cargo test --features math -- --nocapture

use super::*;

#[test]
fn mat3_inverse()
{
    let mut components = [0.0; 9];
    fn perm(components: &mut [f32; 9], i: usize)
    {
        for j in 0..2
        {
            let c = (j as f32 - 0.5) * (i as f32 + 1.0);
            components[i] = c;
            if i + 1 < components.len()
            {
                perm(components, i + 1);
            } else
            {
                let mat = Mat3(Vec3(components[0], components[1], components[2]), Vec3(components[3], components[4], components[5]), Vec3(components[6], components[7], components[8]));
                let inv = mat.inverse();
                let a = mat * inv;
                let b = inv * mat;
                println!("{a}\n{b}");
            }
        }
    }
    perm(&mut components, 0);
}

/*
#[test]
fn rotor_from_plane()
{
    for i in 0..9
    {
        let x = i as f32 / 4.5 - 1.0;
        for j in 0..9
        {
            let y = j as f32 / 4.5 - 1.0;
            for k in 0..9
            {
                let z = k as f32 / 4.5 - 1.0;
                let target = Vec3(x, y, z).unit();
                for source in [Vec3::e_x(), Vec3::e_y(), Vec3::e_z()]
                {
                    let rot = Rotor::from_plane(source, target);
                    let result = rot.transform(source);
                    println!("source: {}, target: {}, result: {}", Vec3Prec(source, 1), Vec3Prec(target, 3), Vec3Prec(result, 3));
                }
            }
        }
    }
}
*/

struct Vec3Prec(Vec3, usize);

impl Display for Vec3Prec
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result
    {
        write!(f, "({1:0$}, {2:0$}, {3:0$})", self.1, self.0.0, self.0.1, self.0.2)
    }
}

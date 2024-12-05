use ggez::glam::Vec2;

pub(crate) trait RotateBy
{
    fn rotate_by(self, theta: f32) -> Self;
}

impl RotateBy for Vec2
{
    fn rotate_by(self, theta: f32) -> Self 
    {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Vec2::new(
            self.x * cos_theta - self.y * sin_theta,
            self.x * sin_theta + self.y * cos_theta
        )
    }
}

pub(crate) trait Flip
{
    fn flip_x(self) -> Self;
    fn flip_y(self) -> Self;
}

impl Flip for Vec2
{
    fn flip_x(mut self) -> Self 
    {
        self.x = -self.x;
        self    
    }

    fn flip_y(mut self) -> Self {
        self.y = -self.y;
        self
    }
}

#[cfg(test)]
mod test
{
    use super::*;
    use ggez::glam::Vec2;
    use std::f32::consts::PI;

    fn compare(actual: Vec2, expected: Vec2)
    {
        let diff = (actual - expected).abs();

        assert!(diff.x < f32::EPSILON);
        assert!(diff.y < f32::EPSILON);
    }

    #[test]
    fn rotate_unit_x_by_180()
    {
        let x = Vec2::X;
        let r = x.rotate_by(PI);

        let expected = Vec2::NEG_X;
        compare(r, expected);
    }

    #[test]
    fn rotate_unit_x_by_90()
    {
        let x = Vec2::X;
        let r = x.rotate_by(std::f32::consts::FRAC_PI_2);

        let expected = Vec2::Y;
        compare(r, expected);
    }
}
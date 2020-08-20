use crate::datatype::continuous::*;

pub enum SNFloatNormaliser{
    Sawtooth,
    Triangle,
    Sigmoid,
    Clamp,
    Fractional
}

impl SNFloatNormaliser{
    pub fn normalise(self, value: f32) -> SNFloat
    {
        use SNFloatNormaliser::*;

        match (self)
        {
            Sawtooth => SNFloat::new_sawtooth(value),
            Triangle => SNFloat::new_triangle(value),
            Sigmoid => SNFloat::new_tanh(value),
            Clamp => SNFloat::new_clamped(value),
            Fractional => SNFloat::new_fractional(value),
        }
    }
}
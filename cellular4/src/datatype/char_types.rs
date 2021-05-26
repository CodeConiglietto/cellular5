pub enum DiscreteRotation{
    0,
    90,
    180,
    270
}

impl DiscreteRotation{
    pub fn translate_coord(self, x: usize, y: usize) -> (usize, usize)
    {
        match self {
            DiscreteRotation::0 => (),
            DiscreteRotation::90 => (),
            DiscreteRotation::180 => (),
            DiscreteRotation::270 => (),
        }
    }
}

pub enum DiscreteMirror{
    X,
    Y
}

pub impl DiscreteMirror{
    pub fn translate_coord(self, x: usize, y: usize) -> (usize, usize)
    {
        match self {
            DiscreteMirror::X => ,
            DiscreteMirror::Y => ,
        }
    }
}

pub struct FontChar {
    index: usize,
    fore_color: GenericColor,
    back_color: GenericColor,
    rotation: DiscreteRotation,
    mirror: DiscreteMirror
}

pub struct Font {
    texture: Image,//Some pixel indexable font image
    char_width: usize,
    char_height: usize,
}

pub struct CharBuffer {
    buffer: Array2<FontChar>,
}

pub impl CharBuffer {
    pub fn new(font: Font, x_scalar: usize, y_scalar: usize, px_width: usize, px_height: usize) -> Self {
        //Make a new buffer, with scalars being multipliers for char pixel coords
    }

    pub fn getBufferDimensions(self) -> (usize, usize)
    {
        //return the dimensions of the internal buffer
    }

    pub fn getPixelAtCoordinate(self, coord: CoordinateSet) -> Option<GenericColor> 
    {
        //remember to pad outside of buffer to center the pixels, if outside of frame return None
        //return option so a default generic color node can be provided if out of frame
    }
}
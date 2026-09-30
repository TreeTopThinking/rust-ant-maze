use crate::tex::TexKind;
use macroquad::prelude::*;

pub struct Assets {
    pub dirt_1: Texture2D,
    pub dirt_2: Texture2D,
    pub dirt_3: Texture2D,

    pub grass_1: Texture2D,
    pub grass_2: Texture2D,
    pub grass_3: Texture2D,

    pub background_1: Texture2D,
    pub background_2: Texture2D,
    pub background_3: Texture2D,

    pub mushroom: Texture2D,
    pub rock: Texture2D,
    pub bush_left: Texture2D,
    pub bush_right: Texture2D,
}

impl Assets {
    pub fn new(
        dirt_1_tex: Texture2D,
        dirt_2_tex: Texture2D,
        dirt_3_tex: Texture2D,
        grass_1_tex: Texture2D,
        grass_2_tex: Texture2D,
        grass_3_tex: Texture2D,
        background_1_tex: Texture2D,
        background_2_tex: Texture2D,
        background_3_tex: Texture2D,
        mushroom_tex: Texture2D,
        rock_tex: Texture2D,
        bush_left_tex: Texture2D,
        bush_right_tex: Texture2D,
    ) -> Self {
        Assets {
            dirt_1: dirt_1_tex,
            dirt_2: dirt_2_tex,
            dirt_3: dirt_3_tex,
            grass_1: grass_1_tex,
            grass_2: grass_2_tex,
            grass_3: grass_3_tex,
            background_1: background_1_tex,
            background_2: background_2_tex,
            background_3: background_3_tex,
            mushroom: mushroom_tex,
            rock: rock_tex,
            bush_left: bush_left_tex,
            bush_right: bush_right_tex,
        }
    }

    pub fn find(&self, tex: TexKind) -> &Texture2D {
        match tex {
            TexKind::Dirt1 => &self.dirt_1,
            TexKind::Dirt2 => &self.dirt_2,
            TexKind::Dirt3 => &self.dirt_3,
            TexKind::Grass1 => &self.grass_1,
            TexKind::Grass2 => &self.grass_2,
            TexKind::Grass3 => &self.grass_3,
            TexKind::Background1 => &self.background_1,
            TexKind::Background2 => &self.background_2,
            TexKind::Background3 => &self.background_3,
            TexKind::Mushroom => &self.mushroom,
            TexKind::Rock => &self.rock,
            TexKind::BushLeft => &self.bush_left,
            TexKind::BushRight => &self.bush_right,
        }
    }
}

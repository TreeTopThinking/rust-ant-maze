use crate::tex::Tex;
use raylib::prelude::*;

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

    pub fn find(&self, tex: Tex) -> &Texture2D {
        match tex {
            Tex::Dirt1 => &self.dirt_1,
            Tex::Dirt2 => &self.dirt_2,
            Tex::Dirt3 => &self.dirt_3,
            Tex::Grass1 => &self.grass_1,
            Tex::Grass2 => &self.grass_2,
            Tex::Grass3 => &self.grass_3,
            Tex::Background1 => &self.background_1,
            Tex::Background2 => &self.background_2,
            Tex::Background3 => &self.background_3,
            Tex::Mushroom => &self.mushroom,
            Tex::Rock => &self.rock,
            Tex::BushLeft => &self.bush_left,
            Tex::BushRight => &self.bush_right,
        }
    }
}

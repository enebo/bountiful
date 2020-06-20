use amethyst::assets::Handle;
use amethyst::renderer::SpriteSheet;

pub struct Items {
    pub textures: Handle<SpriteSheet>,
    pub items: Vec<Item>,
}

pub struct Item {
    pub(crate) name: String,
    pub(crate) texture_id: usize,
}

impl Items {
    pub fn new(textures: Handle<SpriteSheet>) -> Self {
        Self {
            textures,
            items: vec![],
        }
    }

    // FIXME: Support for animated sprites
    pub fn add(&mut self, name: String, texture_id: usize) {
        // FIXME: Add sanity to make sure texture_id is not outside of range of textures.
        self.items.push(Item::new(name, texture_id));
    }
}

impl Item {
    pub fn new(name: String, texture_id: usize) -> Self {
        Self {
            name,
            texture_id,
        }
    }
}
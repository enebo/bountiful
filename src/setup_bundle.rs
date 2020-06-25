use amethyst::core::SystemBundle;
use amethyst::ecs::{DispatcherBuilder, World};
use amethyst::Result;

use crate::resources::Hotbar;

pub struct SetupBundle;

/// This was made because WelcomeState would run before the game started and some systems
/// would try and run and realize not everything is setup.  Something feels wrong about all this :)
impl<'a, 'b> SystemBundle<'a, 'b> for SetupBundle {
    fn build(self, world: &mut World, _builder: &mut DispatcherBuilder<'a, 'b>) -> Result<()> {
        world.insert(Hotbar::default()); // will be reinserted later for reals
        Ok(())
    }
}


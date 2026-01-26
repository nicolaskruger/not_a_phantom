use bevy_ecs::component::Component;

use crate::entity::layer::Layer;

pub struct FirstLayerImpl {}

impl Layer for FirstLayerImpl {
    fn dispose(&self) {
        todo!()
    }

    fn reset(&self) {
        todo!()
    }

    fn load(&self) {
        todo!()
    }
}

#[derive(Component)]
pub struct FirstLayer {
    pub layer: Box<dyn Layer>,
}

impl Layer for FirstLayer {
    fn dispose(&self) {
        self.layer.dispose();
    }
    fn reset(&self) {
        self.layer.reset();
    }
    fn load(&self) {
        self.layer.reset();
    }
}

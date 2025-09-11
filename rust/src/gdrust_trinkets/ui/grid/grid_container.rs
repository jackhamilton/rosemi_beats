use godot::obj::WithBaseField;
use godot::classes::IContainer;
use godot::prelude::*;
use godot::classes::InputEventMouseButton;

pub trait ObjectContainer: IContainer + WithBaseField {
    // This prevents the Cursor from handling events that the container will later handle
    fn will_handle(&self, event: Gd<InputEventMouseButton>) -> bool {
        let base = self.base();
        godot_print!("{}, {}", base.get_global_rect(), event.get_global_position());
        base.get_global_rect().contains_point(event.get_global_position())
    }
}

pub struct ContainerSystem<T> where T: ObjectContainer {
    pub containers: Vec<Gd<T>>
}

impl<T: ObjectContainer> Default for ContainerSystem<T> {
    fn default() -> Self {
        ContainerSystem { containers: Vec::<Gd<T>>::new() }
    }
}

impl<T> ContainerSystem<T> where T: ObjectContainer {
    pub fn handled(&self, event: Gd<InputEventMouseButton>) -> bool {
        self.containers.iter()
            .any(|x| x.bind().will_handle(event.clone()))
    }
}

pub unsafe fn yolo<'a, T>(t: &'a T) -> &'static T {
  std::mem::transmute(t)
}

// use shipyard::{World, AllStoragesView, AllStoragesViewMut, System, AllSystem};

// pub trait WorldExt {
//   fn run_with_data_multiple<T: Clone + Copy, B, S: System<(T,), B> + AllSystem<(T,), B>, const SIZE: usize>(&mut self, systems: [S; SIZE], data: T);
// }

// impl<'a> WorldExt for AllStoragesView<'a> {
//   fn run_with_data_multiple<T: Clone + Copy, B, S: System<(T,), B> + AllSystem<(T,), B>, const SIZE: usize>(&mut self, systems: [S; SIZE], data: T) {
//     for system in systems {
//       self.run_with_data(system, data);
//     }
//   }
// }

// impl<'a> WorldExt for AllStoragesViewMut<'a> {
//   fn run_with_data_multiple<T: Clone + Copy, B, S: System<(T,), B> + AllSystem<(T,), B>, const SIZE: usize>(&mut self, systems: [S; SIZE], data: T) {
//     for system in systems {
//       self.run_with_data(system, data);
//     }
//   }
// }

// impl WorldExt for World {
//   fn run_with_data_multiple<T: Clone + Copy, B, S: System<(T,), B> + AllSystem<(T,), B>, const SIZE: usize>(&mut self, systems: [S; SIZE], data: T) {
//     for system in systems {
//       self.run_with_data(system, data);
//     }
//   }
// }

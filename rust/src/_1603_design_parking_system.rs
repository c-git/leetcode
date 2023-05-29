struct ParkingSystem {
    spaces: [i32; 3],
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ParkingSystem {
    fn new(big: i32, medium: i32, small: i32) -> Self {
        Self {
            spaces: [big, medium, small],
        }
    }

    fn add_car(&mut self, car_type: i32) -> bool {
        let space = &mut self.spaces[car_type as usize - 1];
        if *space > 0 {
            *space -= 1;
            true
        } else {
            false
        }
    }
}

/**
 * Your ParkingSystem object will be instantiated and called as such:
 * let obj = ParkingSystem::new(big, medium, small);
 * let ret_1: bool = obj.add_car(carType);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case() {
        let mut parkingSystem = ParkingSystem::new(1, 1, 0);
        assert!(parkingSystem.add_car(1)); // return true because there is 1 available slot for a big car
        assert!(parkingSystem.add_car(2)); // return true because there is 1 available slot for a medium car
        assert!(!parkingSystem.add_car(3)); // return false because there is no available slot for a small car
        assert!(!parkingSystem.add_car(1)); // return false because there is no available slot for a big car. It is already occupied.
    }
}

use std::collections::HashMap;

#[derive(Default)]
struct RunningAverage {
    curr_avg: f64,
    count: f64,
}

impl RunningAverage {
    fn new() -> Self {
        Self {
            curr_avg: 0.0,
            count: 0.0,
        }
    }

    fn add_value(&mut self, value: f64) {
        // Based on https://github.com/c-git/opylib/blob/main/src/opylib/streaming.py
        // (curr_avg * n + new_value) / (n + 1)
        self.curr_avg = (self.curr_avg * self.count + value) / (self.count + 1.0);
        self.count += 1.0;
    }
}

struct UndergroundSystem {
    checked_in_customers: HashMap<i32, (String, i32)>,
    route_averages: HashMap<(String, String), RunningAverage>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {
    fn new() -> Self {
        Self {
            checked_in_customers: Default::default(),
            route_averages: Default::default(),
        }
    }

    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        debug_assert!(!self.checked_in_customers.contains_key(&id));
        self.checked_in_customers.insert(id, (station_name, t));
    }

    fn check_out(&mut self, id: i32, station_name: String, t: i32) {
        let (check_in_station, check_in_time) = self
            .checked_in_customers
            .remove(&id)
            .expect("Constraint: methods are consistent");
        let route_key = (check_in_station, station_name);
        let travel_time = t - check_in_time;
        let running_average = self.route_averages.entry(route_key).or_default();
        running_average.add_value(travel_time as f64);
    }

    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let route_key = (start_station, end_station);
        self.route_averages
            .get(&route_key)
            .expect("Constraint: There will be at least one customer that has traveled")
            .curr_avg
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut underground_system = UndergroundSystem::new();
        underground_system.check_in(45, "Leyton".into(), 3);
        underground_system.check_in(32, "Paradise".into(), 8);
        underground_system.check_in(27, "Leyton".into(), 10);
        underground_system.check_out(45, "Waterloo".into(), 15); // Customer 45 "Leyton" -> "Waterloo" in 15-3 = 12
        underground_system.check_out(27, "Waterloo".into(), 20); // Customer 27 "Leyton" -> "Waterloo" in 20-10 = 10
        underground_system.check_out(32, "Cambridge".into(), 22); // Customer 32 "Paradise" -> "Cambridge" in 22-8 = 14
        assert_eq!(
            underground_system.get_average_time("Paradise".into(), "Cambridge".into()),
            14.0
        ); // return 14.00000. One trip "Paradise" -> "Cambridge", (14) / 1 = 14
        assert_eq!(
            underground_system.get_average_time("Leyton".into(), "Waterloo".into()),
            11.0
        ); // return 11.00000. Two trips "Leyton" -> "Waterloo", (10 + 12) / 2 = 11
        underground_system.check_in(10, "Leyton".into(), 24);
        assert_eq!(
            underground_system.get_average_time("Leyton".into(), "Waterloo".into()),
            11.0
        ); // return 11.00000
        underground_system.check_out(10, "Waterloo".into(), 38); // Customer 10 "Leyton" -> "Waterloo" in 38-24 = 14
        assert_eq!(
            underground_system.get_average_time("Leyton".into(), "Waterloo".into()),
            12.0
        ); // return 12.00000. Three trips "Leyton" -> "Waterloo", (10 + 12 + 14) / 3 = 12
    }

    #[test]
    fn case2() {
        let mut underground_system = UndergroundSystem::new();
        underground_system.check_in(10, "Leyton".into(), 3);
        underground_system.check_out(10, "Paradise".into(), 8); // Customer 10 "Leyton" -> "Paradise" in 8-3 = 5
        assert_eq!(
            underground_system.get_average_time("Leyton".into(), "Paradise".into()),
            5.0
        ); // return 5.00000, (5) / 1 = 5
        underground_system.check_in(5, "Leyton".into(), 10);
        underground_system.check_out(5, "Paradise".into(), 16); // Customer 5 "Leyton" -> "Paradise" in 16-10 = 6
        assert_eq!(
            underground_system.get_average_time("Leyton".into(), "Paradise".into()),
            5.5
        ); // return 5.50000, (5 + 6) / 2 = 5.5
        underground_system.check_in(2, "Leyton".into(), 21);
        underground_system.check_out(2, "Paradise".into(), 30); // Customer 2 "Leyton" -> "Paradise" in 30-21 = 9
        assert!(
            (underground_system.get_average_time("Leyton".into(), "Paradise".into()) - 6.66667)
                .abs()
                <= 0.00001
        ); // return 6.66667, (5 + 6 + 9) / 3 = 6.66667
    }
}

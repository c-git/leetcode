//! Solution for https://leetcode.com/problems/my-calendar-i
//! 729. My Calendar I

use std::collections::BTreeMap;

#[derive(Default)]
struct MyCalendar {
    data: BTreeMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyCalendar {
    fn new() -> Self {
        Default::default()
    }

    fn book(&mut self, start_time: i32, end_time: i32) -> bool {
        for (&prev_start, &prev_end) in self.data.range(..end_time) {
            if !(prev_end <= start_time || end_time <= prev_start) {
                return false;
            }
        }
        self.data.insert(start_time, end_time);
        true
    }
}

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual() {
        let mut my_calendar = MyCalendar::new();
        assert!(my_calendar.book(10, 20)); // return True
        assert!(!my_calendar.book(15, 25)); // return False, It can not be booked because time 15 is already booked by another event.
        assert!(my_calendar.book(20, 30)); // return True, The event can be booked, as the first event takes every time less than 20, but not including 20.
    }
}

//! Solution for https://leetcode.com/problems/product-of-the-last-k-numbers
//! 1352. Product of the Last K Numbers

struct ProductOfNumbers {
    /// Never stores 0
    products: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        Self { products: vec![1] }
    }

    fn add(&mut self, num: i32) {
        if num == 0 {
            self.products.clear();
            self.products.push(1);
        } else {
            let value = self.products.last().unwrap_or(&1) * num;
            self.products.push(value);
        }
    }

    fn get_product(&self, k: i32) -> i32 {
        let k = k as usize;
        if k >= self.products.len() {
            0
        } else {
            let divisor = self.products[self.products.len() - k - 1];
            self.products.last().unwrap() / divisor
        }
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */

// << ---------------- Code below here is only for local use ---------------- >>

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn manually_case() {
        let mut product_of_numbers = ProductOfNumbers::new();
        product_of_numbers.add(3); // [3]
        product_of_numbers.add(0); // [3,0]
        product_of_numbers.add(2); // [3,0,2]
        product_of_numbers.add(5); // [3,0,2,5]
        product_of_numbers.add(4); // [3,0,2,5,4]
        assert_eq!(product_of_numbers.get_product(2), 20); // return 20. The product of the last 2 numbers is 5 * 4 = 20
        assert_eq!(product_of_numbers.get_product(3), 40); // return 40. The product of the last 3 numbers is 2 * 5 * 4 = 40
        assert_eq!(product_of_numbers.get_product(4), 0); // return 0. The product of the last 4 numbers is 0 * 2 * 5 * 4 = 0
        product_of_numbers.add(8); // [3,0,2,5,4,8]
        assert_eq!(product_of_numbers.get_product(2), 32); // return 32. The product of the last 2 numbers is 4 * 8 = 32
    }

    #[test]
    fn failed_test_case() {
        let mut product_of_numbers = ProductOfNumbers::new();
        product_of_numbers.add(1);
        assert_eq!(product_of_numbers.get_product(1), 1);
        assert_eq!(product_of_numbers.get_product(1), 1);
        assert_eq!(product_of_numbers.get_product(1), 1);
        product_of_numbers.add(7);
        product_of_numbers.add(6);
        product_of_numbers.add(7);
    }
}

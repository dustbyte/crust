use std::time;

pub struct FrequencyTracker {
    slice: u64,
    counter: time::Instant,
}

impl FrequencyTracker {
    pub fn new(frequency: u64) -> Self {
        Self {
            slice: time::Duration::from_millis(1000 / frequency).as_millis() as u64,
            counter: time::Instant::now(),
        }
    }

    pub fn from_str(frequency: &str) -> Result<Self, String> {
        if let Ok(value) = frequency.to_string().parse::<u64>() {
            Ok(Self::new(value))
        } else {
            Err(format!(
                "Couldn't parse value '{}' as a frequency.",
                frequency
            ))
        }
    }

    pub fn burnt_duration(&self) -> i128 {
        let result = self.slice as i128 - self.counter.elapsed().as_millis() as i128;

        if result < 0 {
            -result
        } else {
            0
        }
    }

    pub fn is_burnt(&self) -> bool {
        self.burnt_duration() > 0
    }

    pub fn reset(&mut self) {
        self.counter = time::Instant::now()
    }
}

#[cfg(test)]
mod frequency_tracker_test {
    use super::*;
    use std::thread;

    #[test]
    fn new_ft_test() {
        let ft = FrequencyTracker::new(500);

        assert_eq!(ft.slice, 2);

        let ft = FrequencyTracker::new(60);

        assert_eq!(ft.slice, 16);
    }

    #[test]
    fn new_ft_from_str_test() {
        let ft = FrequencyTracker::from_str("500").unwrap();
        assert_eq!(ft.slice, 2);

        let ft = FrequencyTracker::from_str("60").unwrap();
        assert_eq!(ft.slice, 16);

        let ft = FrequencyTracker::from_str("a");
        assert!(ft.is_err());
    }

    #[test]
    fn burnt_duration() {
        let ft = FrequencyTracker::new(500);

        thread::sleep(time::Duration::from_millis(3));

        assert!(ft.burnt_duration() > 0);

        let ft = FrequencyTracker::new(60);
        assert!(ft.burnt_duration() == 0);
    }

    #[test]
    fn is_burnt_test() {
        let ft = FrequencyTracker::new(500);

        thread::sleep(time::Duration::from_millis(3));

        assert!(ft.is_burnt());

        let ft = FrequencyTracker::new(60);
        assert!(!ft.is_burnt());
    }

    #[test]
    fn reset_test() {
        let mut ft = FrequencyTracker::new(60);

        thread::sleep(time::Duration::from_millis(10));
        assert!(ft.counter.elapsed().as_millis() >= 10);
        ft.reset();
        assert!(ft.counter.elapsed().as_millis() < 10);
    }
}

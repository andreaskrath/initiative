use rand::distr::{Distribution, Uniform};

fn dice(head: u16, count: usize) -> u16 {
    let mut rng = rand::rng();
    let uniform = Uniform::new_inclusive(1, head).expect("should never fail");

    let mut sum = 0;
    for _ in 0..count {
        sum += uniform.sample(&mut rng);
    }

    sum
}

pub fn d4(count: usize) -> u16 {
    dice(4, count)
}

pub fn d6(count: usize) -> u16 {
    dice(6, count)
}

pub fn d8(count: usize) -> u16 {
    dice(8, count)
}

pub fn d10(count: usize) -> u16 {
    dice(10, count)
}

pub fn d12(count: usize) -> u16 {
    dice(12, count)
}

pub fn d20(count: usize) -> u16 {
    dice(20, count)
}

#[cfg(test)]
mod test {
    use crate::dice::dice;

    #[test]
    fn one_dice() {
        let head = 20;
        let count = 1;
        let expected = 0;
        let actual = dice(head, count);

        assert_ne!(actual, expected);
    }
}

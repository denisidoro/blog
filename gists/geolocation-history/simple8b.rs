// this code is inspired by https://github.com/martin2250/rs-simple8b

const MAX_VALUE: u64 = 1 << 60;

const MODES: [(u8, u8); 16] = [
    (240, 0),
    (120, 0),
    (60, 1),
    (30, 2),
    (20, 3),
    (15, 4),
    (12, 5),
    (10, 6),
    (8, 7),
    (7, 8),
    (6, 10),
    (5, 12),
    (4, 15),
    (3, 20),
    (2, 30),
    (1, 60),
];

fn get_min_selector(i: u64) -> Result<usize, &'static str> {
    if i >= MAX_VALUE {
        return Err("value too large");
    }

    let bits = 64 - i.leading_zeros() as u8;

    let mut mode = 2;
    #[allow(clippy::needless_range_loop)]
    for m in 2..16 {
        mode = m;
        if MODES[m].1 > bits {
            break;
        }
    }

    Ok(mode)
}

fn count_leading_zeroes(data: &[u64]) -> usize {
    let mut n = 0;
    for &v in data.iter() {
        if v != 0 {
            break;
        }
        n += 1;
    }
    n
}

/// Pack as many values from data into a single u64
/// All values of data must be smaller than 2^60
/// Returns the number of values in result or an error if a value is too large
pub fn pack(data: &[u64]) -> Result<(u64, usize), &'static str> {
    let mut selector = 0;
    let mut count: usize = 0;

    let leading_zeroes = count_leading_zeroes(data);
    if leading_zeroes > 60 {
        selector = 1;
        let result = (leading_zeroes as u64) + ((selector as u64) << 60);
        return Ok((result, leading_zeroes));
    }

    // add more values to pool and keep track of selector until the maximum number of bits is exceeded
    for &v in data.iter() {
        let selector_next = usize::max(get_min_selector(v + 1)?, selector);
        if count >= MODES[selector_next].0 as usize {
            break;
        }
        selector = selector_next;
        count += 1;
    }

    let mut packed = 0;
    for &v in data.iter().take(count).rev() {
        packed <<= MODES[selector].1;
        packed |= v + 1;
    }

    let result = packed | (selector as u64) << 60;
    Ok((result, count))
}

/// Count the number of values packed inside the u64
fn count_packed(v: u64) -> u8 {
    MODES[(v >> 60) as usize].0
}

/// Decode values from a packed u64 into output
pub fn unpack(v: u64) -> Vec<u64> {
    let mut output = vec![];
    let mode = (v >> 60) as usize;

    if mode == 0 {
        unimplemented!("mode 0");
    } else if mode == 1 {
        let mask = u64::max_value() >> (64 - 60);
        let count = v & mask;
        output = vec![0; count as usize];
    } else {
        let count = count_packed(v);

        let bits = MODES[mode].1 as u32;
        let mask = u64::max_value() >> (64 - bits);
        let mut v = v;
        for _ in 0..count {
            let value = v & mask;
            if value == 0 {
                break;
            }
            output.push(value - 1);
            v >>= bits;
        }
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        let input = [2, 76, 3, 5, 7, 2];
        let (n, count) = pack(&input).unwrap();
        let output = unpack(n);
        assert_eq!(&input[..], &output[..]);
        assert_eq!(count, output.len());
    }

    #[test]
    fn test_bar() {
        let input = [
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2762,
        ];
        let (n, count) = pack(&input).unwrap();
        let output = unpack(n);
        assert_eq!(count, output.len());
    }

    #[test]
    fn test_mode1() {
        let input = [0; 58634];
        let (n, count) = pack(&input).unwrap();
        let output = unpack(n);
        assert_eq!(count, output.len());
    }
}

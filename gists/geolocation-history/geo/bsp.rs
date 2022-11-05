pub const OPERATIONS: usize = 22;

fn pack(x: f32) -> u32 {
    let mut out: u32 = 0;

    let mut min_x = 0.0;
    let mut max_x = 1.0;

    for n in 0..OPERATIONS {
        let midpoint = min_x + (max_x - min_x) / 2.0;
        if x < midpoint {
            max_x = midpoint;
        } else {
            out += 1 << (31 - n);
            min_x = midpoint;
        }
    }

    out
}

fn unpack(mut x: u32) -> f32 {
    let mut out: f32 = 0.0;

    let mut min_x = 0.0;
    let mut max_x = 1.0;
    let mask = 1 << 31;

    for _ in 0..OPERATIONS {
        let midpoint = min_x + (max_x - min_x) / 2.0;

        let is_bit_set = (x & mask) != 0;
        x <<= 1;
        if is_bit_set {
            out = max_x;
            min_x = midpoint;
        } else {
            out = min_x;
            max_x = midpoint;
        }
    }

    out
}

pub fn pack2d(x: f32, y: f32) -> u64 {
    let mut x_packed = pack(x);
    let mut y_packed = pack(y);

    let mut out: u64 = 0;
    for n in 0..64 {
        let is_horizontal = n % 2 == 0;

        let is_bit_set = if is_horizontal {
            let res = (x_packed & 0b1) != 0;
            x_packed >>= 1;
            res
        } else {
            let res = (y_packed & 0b1) != 0;
            y_packed >>= 1;
            res
        };

        if is_bit_set {
            out += 1 << n;
        }
    }
    out
}

pub fn unpack2d(mut packed: u64) -> (f32, f32) {
    let mut x_packed = 0;
    let mut y_packed = 0;

    for n in 0..64 {
        let is_horizontal = n % 2 == 0;
        let bit = n / 2;
        let is_bit_set = (packed & 0b1) != 0;
        packed >>= 1;
        if is_horizontal && is_bit_set {
            x_packed += 1 << bit;
        } else if !is_horizontal && is_bit_set {
            y_packed += 1 << bit;
        }
    }

    (unpack(x_packed), unpack(y_packed))
}

#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::*;

    fn assert_float(x: f32, y: f32, error: f32) {
        assert_le!((x - y).abs(), error);
    }

    fn as_binary_string(n: f32) -> String {
        format!("{:032b}", pack(n)).chars().take(16).collect()
    }

    #[test]
    fn pack_bits() {
        assert_eq!("0000000000000110", as_binary_string(0.0001));
        assert_eq!("0000000000001101", as_binary_string(0.0002));
        assert_eq!("1000000000000000", as_binary_string(0.500001));
        assert_eq!("1111111111111111", as_binary_string(0.999999));
    }

    #[test]
    fn inverse_1d() {
        let input = 0.3;
        let output = unpack(pack(input));
        assert_float(input, output, 0.0001);
    }

    #[test]
    fn inverse_2d() {
        let input = (0.3, 0.7);
        let output = unpack2d(pack2d(input.0, input.1));
        dbg!((input, output));
        assert_float(input.0, output.0, 0.0001);
        assert_float(input.1, output.1, 0.0001);
    }

    #[test]
    fn distance_2d() {
        let cases = vec![
            ((0.3, 0.7), (0.299, 0.701)),
            ((0.999, 0.001), (1., 0.)),
            ((0.5001, 0.5001), (0.5002, 0.5002)),
        ];

        for (input0, input1) in cases {
            let output0 = pack2d(input0.0, input0.1);
            let output1 = pack2d(input1.0, input1.1);
            let ratio = (output0 as f32) / (output1 as f32);
            assert_ge!(ratio, 0.99);
            assert_le!(ratio, 1.01);
        }
    }
}

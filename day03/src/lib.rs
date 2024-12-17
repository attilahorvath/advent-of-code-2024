pub fn mul(memory: &str, conditions: bool) -> u32 {
    let mut sum = 0;
    let mut index = 0;
    let mut enabled = true;

    loop {
        if index >= memory.len() {
            break;
        }

        let chunk = &memory[index..];

        let start_index = chunk.find("mul(");
        let do_index = chunk.find("do()");
        let dont_index = chunk.find("don't()");

        if start_index.is_none() {
            break;
        }

        let start_index = start_index.unwrap_or_default();
        let offset = start_index + "mul(".len();
        let chunk = &chunk[offset..];

        if let Some(do_index) = do_index {
            if do_index < start_index
                && (dont_index.is_none() || dont_index.unwrap_or_default() > do_index)
            {
                enabled = true;
                index += do_index + "do()".len();

                continue;
            }
        }

        if let Some(dont_index) = dont_index {
            if dont_index < start_index
                && (do_index.is_none() || do_index.unwrap_or_default() > dont_index)
            {
                enabled = false;
                index += dont_index + "don't()".len();

                continue;
            }
        }

        let comma_index = chunk.find(",");

        if comma_index.is_none() {
            index += offset;

            continue;
        }

        let comma_index = comma_index.unwrap_or_default();

        if comma_index == 0 || comma_index > 3 {
            index += offset;

            continue;
        }

        let a = chunk[..comma_index].parse();

        if a.is_err() {
            index += offset;

            continue;
        }

        let a: u32 = a.unwrap_or_default();

        let chunk = &chunk[comma_index + 1..];

        let end_index = chunk.find(")");

        if end_index.is_none() {
            index += offset;

            continue;
        }

        let end_index = end_index.unwrap_or_default();

        if end_index == 0 || end_index > 3 {
            index += offset;

            continue;
        }

        let b = chunk[..end_index].parse();

        if b.is_err() {
            index += offset;

            continue;
        }

        let b: u32 = b.unwrap_or_default();

        index += offset + comma_index + 1 + end_index + 1;

        if !conditions || enabled {
            sum += a * b;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_multiplications() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(161, mul(input, false));
    }

    #[test]
    fn sum_multiplications_with_conditions() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(48, mul(input, true));
    }
}

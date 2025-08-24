#![no_std]

use core::{
    cmp::Ordering::{self, Less},
    ops::Range
};

fn binary_first<T, F>(array: &[T], value: &T, range: Range<usize>, cmp: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering
{
    let mut start = range.start;
    let mut end = range.end - 1;
    if range.start >= range.end {
        return range.start;
    }
    while start < end {
        let mid = start + ((end - start) >> 1);
        if cmp(&array[mid], value) == Less {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    if start == range.end - 1 && cmp(&array[start], value) == Less {
        start += 1;
    }
    start
}

fn binary_last<T, F>(array: &[T], value: &T, range: Range<usize>, cmp: &F) -> usize
where
    F: Fn(&T, &T) -> Ordering
{
    let mut start = range.start;
    let mut end = range.end - 1;
    if range.start >= range.end {
        return range.end;
    }
    while start < end {
        let mid = start + ((end - start) >> 1);
        if cmp(value, &array[mid]) != Less {
            start = mid + 1;
        } else {
            end = mid;
        }
    }
    if start == range.end - 1 && cmp(value, &array[start]) != Less {
        start += 1;
    }
    start
}

fn find_first_forward<T, F>(
    array: &[T],
    value: &T,
    range: Range<usize>,
    cmp: &F,
    unique: usize
) -> usize
where
    F: Fn(&T, &T) -> Ordering
{
    if range.len() == 0 {
        return range.start;
    }
    let skip = (range.len() / unique).max(1);

    let mut index = range.start + skip;
    while index < array.len() && cmp(&array[index - 1], value) == Less {
        if index >= range.end - skip {
            return binary_first(array, value, index..range.end, cmp);
        }
        index += skip;
    }

    binary_first(array, value, index - skip..index, cmp)
}

fn find_last_forward<T, F>(
    array: &[T],
    value: &T,
    range: Range<usize>,
    cmp: &F,
    unique: usize
) -> usize
where
    F: Fn(&T, &T) -> Ordering
{
    if range.len() == 0 {
        return range.start;
    }
    let skip = (range.len() / unique).max(1);

    let mut index = range.start + skip;
    while index < array.len() && cmp(value, &array[index - 1]) != Less {
        if index >= range.end - skip {
            return binary_last(array, value, index..range.end, cmp);
        }
        index += skip;
    }

    binary_last(array, value, index - skip..index, cmp)
}

fn find_first_backward<T, F>(
    array: &[T],
    value: &T,
    range: Range<usize>,
    cmp: &F,
    unique: usize
) -> usize
where
    F: Fn(&T, &T) -> Ordering
{
    if range.len() == 0 {
        return range.start;
    }
    let skip = (range.len() / unique).max(1);

    let mut index = range.end - skip;
    while index > range.start && cmp(&array[index - 1], value) != Less {
        if index < range.start + skip {
            return binary_first(array, value, range.start..index, cmp);
        }
        index -= skip;
    }

    binary_first(array, value, index..index + skip, cmp)
}

fn find_last_backward<T, F>(
    array: &[T],
    value: &T,
    range: Range<usize>,
    cmp: &F,
    unique: usize
) -> usize
where
    F: Fn(&T, &T) -> Ordering
{
    if range.len() == 0 {
        return range.start;
    }
    let skip = (range.len() / unique).max(1);

    let mut index = range.end - skip;
    while index > range.start && cmp(value, &array[index - 1]) == Less {
        if index < range.start + skip {
            return binary_last(array, value, range.start..index, cmp);
        }
        index -= skip;
    }

    binary_last(array, value, index..index + skip, cmp)
}

fn insertion_sort<T, F>(array: &mut [T], range: Range<usize>, cmp: &F)
where
    F: Fn(&T, &T) -> Ordering
{
    for i in (range.start + 1)..range.end {
        let mut j = i;
        while j > range.start && cmp(&array[j], &array[j - 1]) == Less {
            array.swap(j, j - 1);
            j -= 1;
        }
    }
}

fn reverse<T>(array: &mut [T], range: Range<usize>) {
    for index in 0..(range.len() >> 1) {
        array.swap(range.start + index, range.end - index - 1);
    }
}

fn block_swap<T>(array: &mut [T], start1: usize, start2: usize, block_size: usize) {
    for index in 0..block_size {
        array.swap(start1 + index, start2 + index);
    }
}

fn rotate<T>(array: &mut [T], amount: usize, range: Range<usize>) {
    if range.len() == 0 {
        return;
    }

    let split = range.start + amount;
    let range1 = range.start..split;
    let range2 = split..range.end;

    reverse(array, range1);
    reverse(array, range2);
    reverse(array, range);
}

struct WikiIterator {
    size: usize,
    numerator: usize,
    decimal: usize,
    denominator: usize,
    decimal_step: usize,
    numerator_step: usize
}

impl WikiIterator {
    fn begin(&mut self) {
        self.numerator = 0;
        self.decimal = 0;
    }

    fn next_range(&mut self) -> Range<usize> {
        let start = self.decimal;

        self.decimal += self.decimal_step;
        self.numerator += self.numerator_step;
        if self.numerator >= self.denominator {
            self.numerator -= self.denominator;
            self.decimal += 1;
        }

        start..self.decimal
    }

    fn finished(&self) -> bool {
        self.decimal >= self.size
    }

    fn next_level(&mut self) -> bool {
        self.decimal_step += self.decimal_step;
        self.numerator_step += self.numerator_step;
        if self.numerator_step >= self.denominator {
            self.numerator_step -= self.denominator;
            self.decimal_step += 1;
        }

        self.decimal_step < self.size
    }

    fn len(&self) -> usize {
        self.decimal_step
    }

    fn new(size: usize, min_level: usize) -> Self {
        let power_of_two = 1 << size.ilog2();
        let denominator = power_of_two / min_level;
        let numerator_step = size % denominator;
        let decimal_step = size / denominator;

        return Self {
            size,
            numerator: 0,
            decimal: 0,
            denominator,
            decimal_step,
            numerator_step
        };
    }
}

fn merge_internal<T, F>(
    array: &mut [T],
    a: Range<usize>,
    b: Range<usize>,
    cmp: &F,
    buffer: Range<usize>
) where
    F: Fn(&T, &T) -> Ordering
{
    let mut a_count = 0;
    let mut b_count = 0;
    let mut insert = 0;

    if b.len() > 0 && a.len() > 0 {
        loop {
            if cmp(&array[b.start + b_count], &array[buffer.start + a_count]) != Less {
                array.swap(a.start + insert, buffer.start + a_count);
                a_count += 1;
                insert += 1;
                if a_count >= a.len() {
                    break;
                }
            } else {
                array.swap(a.start + insert, b.start + b_count);
                b_count += 1;
                insert += 1;
                if b_count >= b.len() {
                    break;
                }
            }
        }
    }

    block_swap(
        array,
        buffer.start + a_count,
        a.start + insert,
        a.len() - a_count
    );
}

fn merge_in_place<T, F>(array: &mut [T], mut a: Range<usize>, mut b: Range<usize>, cmp: &F)
where
    F: Fn(&T, &T) -> Ordering
{
    if a.len() == 0 || b.len() == 0 {
        return;
    }

    loop {
        let mid = binary_first(array, &array[a.start], b.clone(), cmp);

        let amount = mid - a.end;
        rotate(array, a.len(), a.start..mid);
        if b.end == mid {
            break;
        }

        b.start = mid;
        a = a.start + amount..b.start;
        a.start = binary_last(array, &array[a.start], a.clone(), cmp);
        if a.len() == 0 {
            break;
        }
    }
}

pub fn wikisort<T, F>(array: &mut [T], cmp: F)
where
    F: Fn(&T, &T) -> Ordering
{
    if array.len() < 4 {
        if array.len() == 3 {
            if cmp(&array[1], &array[0]) == Less {
                array.swap(0, 1);
            }
            if cmp(&array[2], &array[1]) == Less {
                array.swap(1, 2);
                if cmp(&array[1], &array[0]) == Less {
                    array.swap(0, 1);
                }
            }
        } else if array.len() == 2 {
            if cmp(&array[1], &array[0]) == Less {
                array.swap(0, 1);
            }
        }
        return;
    }

    let mut iterator = WikiIterator::new(array.len(), 4);
    iterator.begin();
    while !iterator.finished() {
        let mut order: [u8; 8] = [0, 1, 2, 3, 4, 5, 6, 7];
        let range = iterator.next_range();

        macro_rules! swap_net {
            ($x:expr, $y:expr) => {
                if cmp(&array[range.start + $y], &array[range.start + $x]) == Less
                    || (order[$x] > order[$y]
                        && cmp(&array[range.start + $x], &array[range.start + $y]) != Less)
                {
                    array.swap(range.start + $x, range.start + $y);
                    order.swap($x, $y);
                }
            };
        }

        if range.len() == 8 {
            swap_net!(0, 1); swap_net!(2, 3); swap_net!(4, 5); swap_net!(6, 7);
            swap_net!(0, 2); swap_net!(1, 3); swap_net!(4, 6); swap_net!(5, 7);
            swap_net!(1, 2); swap_net!(5, 6); swap_net!(0, 4); swap_net!(3, 7);
            swap_net!(1, 5); swap_net!(2, 6);
            swap_net!(1, 4); swap_net!(3, 6);
            swap_net!(2, 4); swap_net!(3, 5);
            swap_net!(3, 4);
        } else if range.len() == 7 {
            swap_net!(1, 2); swap_net!(3, 4); swap_net!(5, 6);
            swap_net!(0, 2); swap_net!(3, 5); swap_net!(4, 6);
            swap_net!(0, 1); swap_net!(4, 5); swap_net!(2, 6);
            swap_net!(0, 4); swap_net!(1, 5);
            swap_net!(0, 3); swap_net!(2, 5);
            swap_net!(1, 3); swap_net!(2, 4);
            swap_net!(2, 3);
        } else if range.len() == 6 {
            swap_net!(1, 2); swap_net!(4, 5);
            swap_net!(0, 2); swap_net!(3, 5);
            swap_net!(0, 1); swap_net!(3, 4); swap_net!(2, 5);
            swap_net!(0, 3); swap_net!(1, 4);
            swap_net!(2, 4); swap_net!(1, 3);
            swap_net!(2, 3);
        } else if range.len() == 5 {
            swap_net!(0, 1); swap_net!(3, 4);
            swap_net!(2, 4);
            swap_net!(2, 3); swap_net!(1, 4);
            swap_net!(0, 3);
            swap_net!(0, 2); swap_net!(1, 3);
            swap_net!(1, 2);
        } else if range.len() == 4 {
            swap_net!(0, 1); swap_net!(2, 3);
            swap_net!(0, 2); swap_net!(1, 3);
            swap_net!(1, 2);
        }
    }
    if array.len() < 8 {
        return;
    }

    loop {
        let block_size = iterator.len().isqrt();
        let buffer_size = iterator.len() / block_size + 1;

        let mut buffer1 = 0..0;
        let mut buffer2 = 0..0;

        let mut pull_index = 0;
        struct Pull {
            from: usize,
            to: usize,
            count: usize,
            range: Range<usize>
        }
        let mut pull: [Pull; 2] = [
            Pull {
                from: 0,
                to: 0,
                count: 0,
                range: 0..0
            },
            Pull {
                from: 0,
                to: 0,
                count: 0,
                range: 0..0
            }
        ];

        let mut find = buffer_size << 1;
        let mut find_separately = false;

        if find > iterator.len() {
            find = buffer_size;
            find_separately = true;
        }

        iterator.begin();
        while !iterator.finished() {
            let a = iterator.next_range();
            let b = iterator.next_range();
            let mut last = a.start;
            let mut count = 1;
            let mut index;

            macro_rules! pull_to {
                ($to:expr) => {
                    pull[pull_index].range = a.start..b.end;
                    pull[pull_index].count = count;
                    pull[pull_index].from = index;
                    pull[pull_index].to = $to;
                };
            }

            while count < find {
                index = find_last_forward(array, &array[last], last + 1..a.end, &cmp, find - count);
                if index == a.end {
                    break;
                }
                last = index;
                count += 1;
            }
            index = last;

            if count >= buffer_size {
                pull_to!(a.start);
                pull_index = 1;

                if count == buffer_size << 1 {
                    buffer1 = a.start..a.start + buffer_size;
                    buffer2 = a.start + buffer_size..a.start + count;
                    break;
                } else if find == buffer_size << 1 {
                    buffer1 = a.start..a.start + count;
                    find = buffer_size;
                } else if find_separately {
                    buffer1 = a.start..a.start + count;
                    find_separately = false;
                } else {
                    buffer2 = a.start..a.start + count;
                    break;
                }
            } else if pull_index == 0 && count > buffer1.len() {
                buffer1 = a.start..a.start + count;
                pull_to!(a.start);
            }

            last = b.end - 1;
            count = 1;
            while count < find {
                index = find_first_backward(array, &array[last], b.start..last, &cmp, find - count);
                if index == b.start {
                    break;
                }
                last = index - 1;
                count += 1;
            }
            index = last;

            if count >= buffer_size {
                pull_to!(b.end);
                pull_index = 1;

                if count == buffer_size << 1 {
                    buffer1 = b.end - count..b.end - buffer_size;
                    buffer2 = b.end - buffer_size..b.end;
                    break;
                } else if find == buffer_size << 1 {
                    buffer1 = b.end - count..b.end;
                    find = buffer_size;
                } else if find_separately {
                    buffer1 = b.end - count..b.end;
                    find_separately = false;
                } else {
                    if pull[0].range.start == a.start {
                        pull[0].range.end -= pull[1].count;
                    }
                    buffer2 = b.end - count..b.end;
                    break;
                }
            } else if pull_index == 0 && count > buffer1.len() {
                buffer1 = b.end - count..b.end;
                pull_to!(b.end);
            }
        }

        for pull_index in 0..2 {
            let len = pull[pull_index].count;

            if pull[pull_index].to < pull[pull_index].from {
                let mut index = pull[pull_index].from;
                for count in 1..len {
                    index = find_first_backward(
                        array,
                        &array[index - 1],
                        pull[pull_index].to..pull[pull_index].from - (count - 1),
                        &cmp,
                        len - count
                    );
                    let range = index + 1..pull[pull_index].from + 1;
                    rotate(array, range.len() - count, range);
                    pull[pull_index].from = index + count;
                }
            } else if pull[pull_index].to > pull[pull_index].from {
                let mut index = pull[pull_index].from + 1;
                for count in 1..len {
                    index = find_last_forward(
                        array,
                        &array[index],
                        index..pull[pull_index].to,
                        &cmp,
                        len - count
                    );
                    let range = pull[pull_index].from..index - 1;
                    rotate(array, count, range);
                    pull[pull_index].from = index - 1 - count;
                }
            }
        }

        let buffer_size = buffer1.len();
        let block_size = iterator.len() / buffer_size + 1;

        iterator.begin();
        while !iterator.finished() {
            let mut a = iterator.next_range();
            let mut b = iterator.next_range();

            let start = a.start;
            if start == pull[0].range.start {
                if pull[0].from > pull[0].to {
                    a.start += pull[0].count;
                    if a.len() == 0 {
                        continue;
                    }
                } else if pull[0].from < pull[0].to {
                    b.end -= pull[0].count;
                    if b.len() == 0 {
                        continue;
                    }
                }
            }
            if start == pull[1].range.start {
                if pull[1].from > pull[1].to {
                    a.start += pull[1].count;
                    if a.len() == 0 {
                        continue;
                    }
                } else if pull[1].from < pull[1].to {
                    b.end -= pull[1].count;
                    if b.len() == 0 {
                        continue;
                    }
                }
            }

            if cmp(&array[b.end - 1], &array[a.start]) == Less {
                rotate(array, a.len(), a.start..b.end);
            } else if cmp(&array[a.end], &array[a.end - 1]) == Less {
                let mut block_a = a.start..a.end;
                let first_a = a.start..a.start + block_a.len() % block_size;

                let mut index_a = buffer1.start;
                let mut index = first_a.end;
                while index < block_a.end {
                    array.swap(index_a, index);
                    index_a += 1;
                    index += block_size;
                }

                let mut last_a = first_a.clone();
                let mut last_b = 0..0;
                let mut block_b = b.start..b.start + block_size.min(b.len());
                block_a.start += first_a.len();
                index_a = buffer1.start;

                if buffer2.len() > 0 {
                    block_swap(array, last_a.start, buffer2.start, last_a.len());
                }

                if block_a.len() > 0 {
                    loop {
                        if (last_b.len() > 0
                            && cmp(&array[last_b.end - 1], &array[index_a]) != Less)
                            || block_b.len() == 0
                        {
                            let b_split =
                                binary_first(array, &array[index_a], last_b.clone(), &cmp);
                            let b_remaining = last_b.end - b_split;

                            let mut min_a = block_a.start;
                            let mut find_a = min_a + block_size;
                            while find_a < block_a.end {
                                if cmp(&array[find_a], &array[min_a]) == Less {
                                    min_a = find_a;
                                }
                                find_a += block_size;
                            }
                            block_swap(array, block_a.start, min_a, block_size);

                            array.swap(block_a.start, index_a);
                            index_a += 1;

                            if buffer2.len() > 0 {
                                merge_internal(
                                    array,
                                    last_a.clone(),
                                    last_a.end..b_split,
                                    &cmp,
                                    buffer2.clone()
                                );
                            } else {
                                merge_in_place(array, last_a.clone(), last_a.end..b_split, &cmp);
                            }

                            if buffer2.len() > 0 {
                                block_swap(array, block_a.start, buffer2.start, block_size);
                                block_swap(
                                    array,
                                    b_split,
                                    block_a.start + block_size - b_remaining,
                                    b_remaining
                                );
                            } else {
                                rotate(
                                    array,
                                    block_a.start - b_split,
                                    b_split..block_a.start + block_size
                                );
                            }

                            last_a = block_a.start - b_remaining
                                ..block_a.start - b_remaining + block_size;
                            last_b = last_a.end..last_a.end + b_remaining;

                            block_a.start += block_size;
                            if block_a.len() == 0 {
                                break;
                            }
                        } else if block_b.len() < block_size {
                            rotate(
                                array,
                                block_b.start - block_a.start,
                                block_a.start..block_b.end
                            );

                            last_b = block_a.start..block_a.start + block_b.len();
                            block_a.start += block_b.len();
                            block_a.end += block_b.len();
                            block_b.end = block_b.start;
                        } else {
                            block_swap(array, block_a.start, block_b.start, block_size);
                            last_b = block_a.start..block_a.start + block_size;

                            block_a.start += block_size;
                            block_a.end += block_size;
                            block_b.start += block_size;

                            if block_b.end > b.end - block_size {
                                block_b.end = b.end;
                            } else {
                                block_b.end += block_size;
                            }
                        }
                    }
                }

                if buffer2.len() > 0 {
                    merge_internal(
                        array,
                        last_a.clone(),
                        last_a.end..b.end,
                        &cmp,
                        buffer2.clone()
                    );
                } else {
                    merge_in_place(array, last_a.clone(), last_a.end..b.end, &cmp);
                }
            }
        }

        insertion_sort(array, buffer2, &cmp);

        for pull_index in 0..2 {
            let mut unique = pull[pull_index].count << 1;
            if pull[pull_index].from > pull[pull_index].to {
                let mut buffer = pull[pull_index].range.start
                    ..pull[pull_index].range.start + pull[pull_index].count;
                while buffer.len() > 0 {
                    let index = find_first_forward(
                        array,
                        &array[buffer.start],
                        buffer.end..pull[pull_index].range.end,
                        &cmp,
                        unique
                    );
                    let amount = index - buffer.end;
                    rotate(array, buffer.len(), buffer.start..index);
                    buffer.start += amount + 1;
                    buffer.end += amount;
                    unique -= 2;
                }
            } else if pull[pull_index].from < pull[pull_index].to {
                let mut buffer =
                    pull[pull_index].range.end - pull[pull_index].count..pull[pull_index].range.end;
                while buffer.len() > 0 {
                    let index = find_last_backward(
                        array,
                        &array[buffer.end - 1],
                        pull[pull_index].range.start..buffer.start,
                        &cmp,
                        unique
                    );
                    let amount = buffer.start - index;
                    rotate(array, amount, index..buffer.end);
                    buffer.start -= amount;
                    buffer.end -= amount + 1;
                    unique -= 2;
                }
            }
        }

        if !iterator.next_level() {
            break;
        }
    }
}

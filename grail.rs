// this one is MIT licensed, by Andrey Astrelin

// The MIT License (MIT)

// Copyright (c) 2013 Andrey Astrelin

// Permission is hereby granted, free of charge, to any person obtaining a copy of
// this software and associated documentation files (the "Software"), to deal in
// the Software without restriction, including without limitation the rights to
// use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
// the Software, and to permit persons to whom the Software is furnished to do so,
// subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
// FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
// COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
// IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

// yeah blah blah whatever fuck legal notes

use core::{cmp::Ordering, ptr};

unsafe fn grail_multi_swap<T>(mut a: *mut T, mut b: *mut T, mut swaps_left: isize) {
    while swaps_left > 0 {
        ptr::swap(a, b);
        a = a.offset(1);
        b = b.offset(1);
        swaps_left -= 1;
    }
}

fn grail_insert_sort<T, F>(arr: &mut [T], cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && cmp(&arr[j], &arr[j - 1]) == Ordering::Less {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

unsafe fn grail_rotate<T>(mut arr: *mut T, mut len_a: isize, mut len_b: isize) {
    while len_a > 0 && len_b > 0 {
        if len_a <= len_b {
            grail_multi_swap(arr, arr.offset(len_a), len_a);
            arr = arr.offset(len_a);
            len_b -= len_a;
        } else {
            grail_multi_swap(arr.offset(len_a - len_b), arr.offset(len_a), len_b);
            len_a -= len_b;
        }
    }
}

unsafe fn grail_binary_search<T, F>(
    arr: *const T,
    len: isize,
    key: &T,
    dir: bool,
    cmp: &mut F,
) -> isize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut left = -1;
    let mut right = len;

    while left < right - 1 {
        let mid = left + ((right - left) >> 1);
        if !dir {
            match cmp(&*arr.offset(mid), key) {
                Ordering::Greater | Ordering::Equal => right = mid,
                _ => left = mid,
            }
        } else {
            match cmp(&*arr.offset(mid), key) {
                Ordering::Greater => right = mid,
                _ => left = mid,
            }
        }
    }
    right
}

unsafe fn grail_find_keys<T, F>(arr: *mut T, len: isize, key_count: isize, cmp: &mut F) -> isize
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut dist = 1;
    let mut keys_found = 1;
    let mut first_key = 0;

    while dist < len && keys_found < key_count {
        let loc = grail_binary_search(
            arr.offset(first_key),
            keys_found,
            &*arr.offset(dist),
            false,
            cmp,
        );

        if loc == keys_found
            || cmp(&*arr.offset(dist), &*arr.offset(first_key + loc)) != Ordering::Equal
        {
            grail_rotate(
                arr.offset(first_key),
                keys_found,
                dist - (first_key + keys_found),
            );
            first_key = dist - keys_found;
            grail_rotate(arr.offset(first_key + loc), keys_found - loc, 1);
            keys_found += 1;
        }
        dist += 1;
    }
    grail_rotate(arr, first_key, keys_found);
    keys_found
}

unsafe fn grail_merge_without_buffer<T, F>(
    mut arr: *mut T,
    mut len1: isize,
    mut len2: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    if len1 < len2 {
        while len1 != 0 {
            let loc = grail_binary_search(arr.offset(len1), len2, &*arr, false, cmp);

            if loc != 0 {
                grail_rotate(arr, len1, loc);
                arr = arr.offset(loc);
                len2 -= loc;
            }

            if len2 == 0 {
                break;
            }

            loop {
                arr = arr.offset(1);
                len1 -= 1;
                if len1 == 0 || cmp(&*arr, &*arr.offset(len1)) == Ordering::Greater {
                    break;
                }
            }
        }
    } else {
        while len2 != 0 {
            let loc = grail_binary_search(arr, len1, &*arr.offset(len1 + len2 - 1), true, cmp);

            if loc != len1 {
                grail_rotate(arr.offset(loc), len1 - loc, len2);
                len1 = loc;
            }

            if len1 == 0 {
                break;
            }

            loop {
                len2 -= 1;
                if len2 == 0
                    || cmp(&*arr.offset(len1 - 1), &*arr.offset(len1 + len2 - 1))
                        == Ordering::Greater
                {
                    break;
                }
            }
        }
    }
}

unsafe fn grail_merge_left<T, F>(
    arr: *mut T,
    left_len: isize,
    mut right_len: isize,
    mut dist: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut left = 0;
    let mut right = left_len;
    right_len += left_len;

    while right < right_len {
        if left == left_len || cmp(&*arr.offset(left), &*arr.offset(right)) == Ordering::Greater {
            ptr::swap(arr.offset(dist), arr.offset(right));
            right += 1;
        } else {
            ptr::swap(arr.offset(dist), arr.offset(left));
            left += 1;
        }
        dist += 1;
    }
    if dist != left {
        grail_multi_swap(arr.offset(dist), arr.offset(left), left_len - left);
    }
}

unsafe fn grail_merge_right<T, F>(
    arr: *mut T,
    left_len: isize,
    right_len: isize,
    dist: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut merged_pos = left_len + right_len + dist - 1;
    let mut right = left_len + right_len - 1;
    let mut left = left_len - 1;

    while left >= 0 {
        if right < left_len || cmp(&*arr.offset(left), &*arr.offset(right)) == Ordering::Greater {
            ptr::swap(arr.offset(merged_pos), arr.offset(left));
            left -= 1;
        } else {
            ptr::swap(arr.offset(merged_pos), arr.offset(right));
            right -= 1;
        }
        merged_pos -= 1;
    }
    if right != merged_pos {
        while right >= left_len {
            ptr::swap(arr.offset(merged_pos), arr.offset(right));
            merged_pos -= 1;
            right -= 1;
        }
    }
}

unsafe fn grail_smart_merge_with_buffer<T, F>(
    arr: *mut T,
    left_over_len: &mut isize,
    left_over_frag: &mut isize,
    block_len: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut dist = 0 - block_len;
    let mut left = 0;
    let mut right = *left_over_len;
    let mut left_end = right;
    let mut right_end = right + block_len;
    let frag_type = 1 - *left_over_frag;

    while left < left_end && right < right_end {
        let comparison = match cmp(&*arr.offset(left), &*arr.offset(right)) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        } - frag_type;

        if comparison < 0 {
            ptr::swap(arr.offset(dist), arr.offset(left));
            left += 1;
        } else {
            ptr::swap(arr.offset(dist), arr.offset(right));
            right += 1;
        }
        dist += 1;
    }

    if left < left_end {
        *left_over_len = left_end - left;

        while left < left_end {
            left_end -= 1;
            right_end -= 1;
            ptr::swap(arr.offset(left_end), arr.offset(right_end));
        }
    } else {
        *left_over_len = right_end - right;
        *left_over_frag = frag_type;
    }
}

unsafe fn grail_smart_merge_without_buffer<T, F>(
    mut arr: *mut T,
    left_over_len: &mut isize,
    left_over_frag: &mut isize,
    reg_block_len: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    if reg_block_len == 0 {
        return;
    }

    let mut len1 = *left_over_len;
    let mut len2 = reg_block_len;
    let frag_type = 1 - *left_over_frag;

    if len1 != 0 {
        let comparison = match cmp(&*arr.offset(len1 - 1), &*arr.offset(len1)) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        } - frag_type;

        if comparison >= 0 {
            while len1 != 0 {
                let found_len = if frag_type != 0 {
                    grail_binary_search(arr.offset(len1), len2, &*arr, false, cmp)
                } else {
                    grail_binary_search(arr.offset(len1), len2, &*arr, true, cmp)
                };

                if found_len != 0 {
                    grail_rotate(arr, len1, found_len);
                    arr = arr.offset(found_len);
                    len2 -= found_len;
                }

                if len2 == 0 {
                    *left_over_len = len1;
                    return;
                } else {
                    loop {
                        arr = arr.offset(1);
                        len1 -= 1;
                        if len1 == 0 {
                            break;
                        }
                        let comparison = match cmp(&*arr, &*arr.offset(len1)) {
                            Ordering::Less => -1,
                            Ordering::Equal => 0,
                            Ordering::Greater => 1,
                        } - frag_type;
                        if comparison >= 0 {
                            break;
                        }
                    }
                }
            }
        }
    }
    *left_over_len = len2;
    *left_over_frag = frag_type;
}

unsafe fn grail_merge_left_with_extra_buffer<T>(
    arr: *mut T,
    left_end: isize,
    mut right_end: isize,
    mut dist: isize,
    cmp: &mut impl FnMut(&T, &T) -> Ordering,
) {
    let mut left = 0;
    let mut right = left_end;
    right_end += left_end;

    while right < right_end {
        if left == left_end || cmp(&*arr.offset(left), &*arr.offset(right)) == Ordering::Greater {
            ptr::copy_nonoverlapping(arr.offset(right), arr.offset(dist), 1);
            right += 1;
        } else {
            ptr::copy_nonoverlapping(arr.offset(left), arr.offset(dist), 1);
            left += 1;
        }
        dist += 1;
    }
    if dist != left {
        while left < left_end {
            ptr::copy_nonoverlapping(arr.offset(left), arr.offset(dist), 1);
            left += 1;
            dist += 1;
        }
    }
}

unsafe fn grail_smart_merge_with_extra_buffer<T, F>(
    arr: *mut T,
    left_over_len: &mut isize,
    left_over_frag: &mut isize,
    block_len: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut dist = 0 - block_len;
    let mut left = 0;
    let mut right = *left_over_len;
    let mut left_end = right;
    let mut right_end = right + block_len;
    let frag_type = 1 - *left_over_frag;

    while left < left_end && right < right_end {
        let comparison = cmp(&*arr.offset(left), &*arr.offset(right)) as isize - frag_type;

        if comparison < 0 {
            ptr::copy_nonoverlapping(arr.offset(left), arr.offset(dist), 1);
            left += 1;
        } else {
            ptr::copy_nonoverlapping(arr.offset(right), arr.offset(dist), 1);
            right += 1;
        }
        dist += 1;
    }

    if left < left_end {
        *left_over_len = left_end - left;

        while left < left_end {
            left_end -= 1;
            right_end -= 1;
            ptr::copy_nonoverlapping(arr.offset(left_end), arr.offset(right_end), 1);
        }
    } else {
        *left_over_len = right_end - right;
        *left_over_frag = frag_type;
    }
}

unsafe fn grail_merge_buffers_left_with_extra_buffer<T, F>(
    keys_pos: *mut T,
    midkey: *mut T,
    arr: *mut T,
    block_count: isize,
    reg_block_len: isize,
    a_block_count: isize,
    last_len: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    if block_count == 0 {
        let a_block_len = a_block_count * reg_block_len;
        grail_merge_left_with_extra_buffer(arr, a_block_len, last_len, 0 - reg_block_len, cmp);
        return;
    }

    let mut left_over_len = reg_block_len;
    let mut process_index = reg_block_len;
    let mut left_over_frag = if cmp(&*keys_pos, &*midkey) == Ordering::Less {
        0
    } else {
        1
    };
    let mut rest_to_process;

    for key_index in 1..block_count {
        rest_to_process = process_index - left_over_len;
        let next_frag = if cmp(&*keys_pos.offset(key_index), &*midkey) == Ordering::Less {
            0
        } else {
            1
        };

        if next_frag == left_over_frag {
            ptr::copy_nonoverlapping(
                arr.offset(rest_to_process),
                arr.offset(rest_to_process - reg_block_len),
                left_over_len as usize,
            );

            left_over_len = reg_block_len;
        } else {
            grail_smart_merge_with_extra_buffer(
                arr.offset(rest_to_process),
                &mut left_over_len,
                &mut left_over_frag,
                reg_block_len,
                cmp,
            );
        }
        process_index += reg_block_len;
    }

    rest_to_process = process_index - left_over_len;

    if last_len != 0 {
        if left_over_frag != 0 {
            ptr::copy_nonoverlapping(
                arr.offset(rest_to_process),
                arr.offset(rest_to_process - reg_block_len),
                left_over_len as usize,
            );

            rest_to_process = process_index;
            left_over_len = reg_block_len * a_block_count;
        } else {
            left_over_len += reg_block_len * a_block_count;
        }
        grail_merge_left_with_extra_buffer(
            arr.offset(rest_to_process),
            left_over_len,
            last_len,
            0 - reg_block_len,
            cmp,
        );
    } else {
        ptr::copy_nonoverlapping(
            arr.offset(rest_to_process),
            arr.offset(rest_to_process - reg_block_len),
            left_over_len as usize,
        );
    }
}

unsafe fn grail_merge_buffers_left<T, F>(
    keys_pos: *mut T,
    midkey: *mut T,
    arr: *mut T,
    block_count: isize,
    reg_block_len: isize,
    have_buffer: bool,
    a_block_count: isize,
    last_len: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    if block_count == 0 {
        let total_a_len = a_block_count * reg_block_len;

        if have_buffer {
            grail_merge_left(arr, total_a_len, last_len, 0 - reg_block_len, cmp);
        } else {
            grail_merge_without_buffer(arr, total_a_len, last_len, cmp);
        }
        return;
    }

    let mut left_over_len = reg_block_len;
    let mut process_index = reg_block_len;
    let mut left_over_frag = if cmp(&*keys_pos, &*midkey) == Ordering::Less {
        0
    } else {
        1
    };
    let mut rest_to_process;

    for key_index in 1..block_count {
        rest_to_process = process_index - left_over_len;
        let next_frag = if cmp(&*keys_pos.offset(key_index), &*midkey) == Ordering::Less {
            0
        } else {
            1
        };

        if next_frag == left_over_frag {
            if have_buffer {
                grail_multi_swap(
                    arr.offset(rest_to_process - reg_block_len),
                    arr.offset(rest_to_process),
                    left_over_len,
                );
            }

            left_over_len = reg_block_len;
        } else {
            if have_buffer {
                grail_smart_merge_with_buffer(
                    arr.offset(rest_to_process),
                    &mut left_over_len,
                    &mut left_over_frag,
                    reg_block_len,
                    cmp,
                );
            } else {
                grail_smart_merge_without_buffer(
                    arr.offset(rest_to_process),
                    &mut left_over_len,
                    &mut left_over_frag,
                    reg_block_len,
                    cmp,
                );
            }
        }
        process_index += reg_block_len;
    }

    rest_to_process = process_index - left_over_len;

    if last_len != 0 {
        if left_over_frag != 0 {
            if have_buffer {
                grail_multi_swap(
                    arr.offset(rest_to_process - reg_block_len),
                    arr.offset(rest_to_process),
                    left_over_len,
                );
            }

            rest_to_process = process_index;
            left_over_len = reg_block_len * a_block_count;
        } else {
            left_over_len += reg_block_len * a_block_count;
        }

        if have_buffer {
            grail_merge_left(
                arr.offset(rest_to_process),
                left_over_len,
                last_len,
                0 - reg_block_len,
                cmp,
            );
        } else {
            grail_merge_without_buffer(arr.offset(rest_to_process), left_over_len, last_len, cmp);
        }
    } else {
        if have_buffer {
            grail_multi_swap(
                arr.offset(rest_to_process),
                arr.offset(rest_to_process - reg_block_len),
                left_over_len,
            );
        }
    }
}

unsafe fn grail_build_blocks<T, F>(
    mut arr: *mut T,
    len: isize,
    build_len: isize,
    ext_buf: *mut T,
    ext_buf_len: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut build_buf = if build_len < ext_buf_len {
        build_len
    } else {
        ext_buf_len
    };

    while (build_buf & (build_buf - 1)) != 0 {
        build_buf &= build_buf - 1;
    }

    let mut part;

    if build_buf != 0 {
        ptr::copy_nonoverlapping(arr.offset(-build_buf), ext_buf, build_buf as usize);

        let mut dist = 1;
        while dist < len {
            let extra_dist = if cmp(&*arr.offset(dist - 1), &*arr.offset(dist)) == Ordering::Greater
            {
                1
            } else {
                0
            };

            ptr::copy_nonoverlapping(arr.offset(dist - 1 + extra_dist), arr.offset(dist - 3), 1);
            ptr::copy_nonoverlapping(arr.offset(dist - extra_dist), arr.offset(dist - 2), 1);

            dist += 2;
        }
        if len % 2 != 0 {
            ptr::copy_nonoverlapping(arr.offset(len - 1), arr.offset(len - 3), 1);
        }

        arr = arr.offset(-2);
        part = 2;

        while part < build_buf {
            let mut left = 0;
            let right = len - 2 * part;

            while left <= right {
                grail_merge_left_with_extra_buffer(arr.offset(left), part, part, -part, cmp);
                left += 2 * part;
            }

            let rest = len - left;
            if rest > part {
                grail_merge_left_with_extra_buffer(arr.offset(left), part, rest - part, -part, cmp);
            } else {
                while left < len {
                    ptr::copy_nonoverlapping(arr.offset(left), arr.offset(left - part), 1);
                    left += 1;
                }
            }
            arr = arr.offset(-part);
            part *= 2;
        }
        ptr::copy_nonoverlapping(ext_buf, arr.offset(len), build_buf as usize);
    } else {
        let mut dist = 1;
        while dist < len {
            let extra_dist = if cmp(&*arr.offset(dist - 1), &*arr.offset(dist)) == Ordering::Greater
            {
                1
            } else {
                0
            };

            ptr::swap(arr.offset(dist - 3), arr.offset(dist - 1 + extra_dist));
            ptr::swap(arr.offset(dist - 2), arr.offset(dist - extra_dist));

            dist += 2;
        }

        if len % 2 != 0 {
            ptr::swap(arr.offset(len - 1), arr.offset(len - 3));
        }

        arr = arr.offset(-2);
        part = 2;
    }

    while part < build_len {
        let mut left = 0;
        let right = len - 2 * part;

        while left <= right {
            grail_merge_left(arr.offset(left), part, part, -part, cmp);
            left += 2 * part;
        }

        let rest = len - left;
        if rest > part {
            grail_merge_left(arr.offset(left), part, rest - part, -part, cmp);
        } else {
            grail_rotate(arr.offset(left - part), part, rest);
        }

        arr = arr.offset(-part);
        part *= 2;
    }

    let rest_to_build = len % (2 * build_len);
    let left_over_pos = len - rest_to_build;

    if rest_to_build <= build_len {
        grail_rotate(arr.offset(left_over_pos), rest_to_build, build_len);
    } else {
        grail_merge_right(
            arr.offset(left_over_pos),
            build_len,
            rest_to_build - build_len,
            build_len,
            cmp,
        );
    }

    let mut left_over_pos = left_over_pos;
    while left_over_pos > 0 {
        left_over_pos -= 2 * build_len;
        grail_merge_right(
            arr.offset(left_over_pos),
            build_len,
            build_len,
            build_len,
            cmp,
        );
    }
}

unsafe fn grail_combine_blocks<T, F>(
    keys_pos: *mut T,
    arr: *mut T,
    mut len: isize,
    build_len: isize,
    reg_block_len: isize,
    have_buffer: bool,
    ext_buf: *mut T,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    let combined_len = len / (2 * build_len);
    let mut left_over = len % (2 * build_len);

    if left_over <= build_len {
        len -= left_over;
        left_over = 0;
    }

    if !ext_buf.is_null() {
        ptr::copy_nonoverlapping(arr.offset(-reg_block_len), ext_buf, reg_block_len as usize);
    }

    for i in 0..=combined_len {
        if i == combined_len && left_over == 0 {
            break;
        }

        let block_pos = arr.offset(i * 2 * build_len);
        let block_count = if i == combined_len {
            left_over
        } else {
            2 * build_len
        } / reg_block_len;

        for idx in 1..(block_count + if i == combined_len { 1 } else { 0 }) {
            let mut j = idx;
            while j > 0 && cmp(&*keys_pos.offset(j), &*keys_pos.offset(j - 1)) == Ordering::Less {
                ptr::swap(keys_pos.offset(j - 1), keys_pos.offset(j));
                j -= 1;
            }
        }

        let mut midkey = build_len / reg_block_len;

        for index in 1..block_count {
            let mut left_index = index - 1;

            for right_index in index..block_count {
                let right_comp = cmp(
                    &*block_pos.offset(left_index * reg_block_len),
                    &*block_pos.offset(right_index * reg_block_len),
                );

                if right_comp == Ordering::Greater
                    || (right_comp == Ordering::Equal
                        && cmp(
                            &*keys_pos.offset(left_index),
                            &*keys_pos.offset(right_index),
                        ) == Ordering::Greater)
                {
                    left_index = right_index;
                }
            }

            if left_index != index - 1 {
                grail_multi_swap(
                    block_pos.offset((index - 1) * reg_block_len),
                    block_pos.offset(left_index * reg_block_len),
                    reg_block_len,
                );
                ptr::swap(keys_pos.offset(index - 1), keys_pos.offset(left_index));

                if midkey == index - 1 || midkey == left_index {
                    midkey ^= (index - 1) ^ left_index;
                }
            }
        }

        let mut a_block_count = 0;
        let last_len = if i == combined_len {
            left_over % reg_block_len
        } else {
            0
        };

        if last_len != 0 {
            while a_block_count < block_count
                && cmp(
                    &*block_pos.offset(block_count * reg_block_len),
                    &*block_pos.offset((block_count - a_block_count - 1) * reg_block_len),
                ) == Ordering::Less
            {
                a_block_count += 1;
            }
        }

        if !ext_buf.is_null() {
            grail_merge_buffers_left_with_extra_buffer(
                keys_pos,
                keys_pos.offset(midkey),
                block_pos,
                block_count - a_block_count,
                reg_block_len,
                a_block_count,
                last_len,
                cmp,
            );
        } else {
            grail_merge_buffers_left(
                keys_pos,
                keys_pos.offset(midkey),
                block_pos,
                block_count - a_block_count,
                reg_block_len,
                have_buffer,
                a_block_count,
                last_len,
                cmp,
            );
        }
    }

    if !ext_buf.is_null() {
        for i in (0..len).rev() {
            ptr::copy_nonoverlapping(arr.offset(i - reg_block_len), arr.offset(i), 1);
        }
        ptr::copy_nonoverlapping(ext_buf, arr.offset(-reg_block_len), reg_block_len as usize);
    } else if have_buffer {
        let mut len = len;
        while len > 0 {
            len -= 1;
            ptr::swap(arr.offset(len), arr.offset(len - reg_block_len));
        }
    }
}

unsafe fn grail_lazy_stable_sort<T, F>(arr: *mut T, len: isize, cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut dist = 1;
    while dist < len {
        if cmp(&*arr.offset(dist - 1), &*arr.offset(dist)) == Ordering::Greater {
            ptr::swap(arr.offset(dist - 1), arr.offset(dist));
        }
        dist += 2;
    }

    let mut part = 2;
    while part < len {
        let mut left = 0;
        let right = len - 2 * part;

        while left <= right {
            grail_merge_without_buffer(arr.offset(left), part, part, cmp);
            left += 2 * part;
        }

        let rest = len - left;
        if rest > part {
            grail_merge_without_buffer(arr.offset(left), part, rest - part, cmp);
        }
        part *= 2;
    }
}

unsafe fn grail_common_sort<T, F>(
    arr: *mut T,
    len: isize,
    ext_buf: *mut T,
    ext_buf_len: isize,
    cmp: &mut F,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    if len <= 16 {
        let slice = core::slice::from_raw_parts_mut(arr, len as usize);
        grail_insert_sort(slice, cmp);
        return;
    }

    let mut block_len = 1;
    while block_len * block_len < len {
        block_len *= 2;
    }

    let key_count = (len - 1) / block_len + 1;
    let keys_found = grail_find_keys(arr, len, key_count + block_len, cmp);

    let mut buffer_enabled = true;
    let mut key_count = key_count;

    if keys_found < key_count + block_len {
        if keys_found < 4 {
            grail_lazy_stable_sort(arr, len, cmp);
            return;
        }

        key_count = block_len;
        while key_count > keys_found {
            key_count /= 2;
        }

        buffer_enabled = false;
        block_len = 0;
    }

    let dist = block_len + key_count;
    let mut build_len = if buffer_enabled { block_len } else { key_count };

    if buffer_enabled {
        grail_build_blocks(
            arr.offset(dist),
            len - dist,
            build_len,
            ext_buf,
            ext_buf_len,
            cmp,
        );
    } else {
        grail_build_blocks(
            arr.offset(dist),
            len - dist,
            build_len,
            ptr::null_mut(),
            0,
            cmp,
        );
    }

    while len - dist > {
        build_len *= 2;
        build_len
    } {
        let mut reg_block_len = block_len;
        let mut build_buf_enabled = buffer_enabled;

        if !buffer_enabled {
            if key_count > 4 && key_count / 8 * key_count >= build_len {
                reg_block_len = key_count / 2;
                build_buf_enabled = true;
            } else {
                let mut calc_keys = 1;
                let mut i = (build_len * keys_found) / 2;

                while calc_keys < key_count && i != 0 {
                    calc_keys *= 2;
                    i /= 8;
                }
                reg_block_len = (2 * build_len) / calc_keys;
            }
        }

        let ext_buf_to_use = if build_buf_enabled && reg_block_len <= ext_buf_len {
            ext_buf
        } else {
            ptr::null_mut()
        };

        grail_combine_blocks(
            arr,
            arr.offset(dist),
            len - dist,
            build_len,
            reg_block_len,
            build_buf_enabled,
            ext_buf_to_use,
            cmp,
        );
    }

    let dist_slice = core::slice::from_raw_parts_mut(arr, dist as usize);
    grail_insert_sort(dist_slice, cmp);
    grail_merge_without_buffer(arr, dist, len - dist, cmp);
}

pub fn grail_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut cmp = |a: &T, b: &T| a.cmp(b);
    unsafe {
        grail_common_sort(
            arr.as_mut_ptr(),
            arr.len() as isize,
            ptr::null_mut(),
            0,
            &mut cmp,
        );
    }
}

pub fn grail_sort_by<T, F>(arr: &mut [T], mut compare: F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if arr.is_empty() {
        return;
    }

    unsafe {
        grail_common_sort(
            arr.as_mut_ptr(),
            arr.len() as isize,
            ptr::null_mut(),
            0,
            &mut compare,
        );
    }
}

pub fn grail_sort_with_static_buffer<T: Ord>(arr: &mut [T]) {
    const STATIC_BUFFER_LENGTH: usize = 512;
    if arr.is_empty() {
        return;
    }

    let mut ext_buffer: [core::mem::MaybeUninit<T>; STATIC_BUFFER_LENGTH] =
        unsafe { core::mem::MaybeUninit::uninit().assume_init() };

    let mut cmp = |a: &T, b: &T| a.cmp(b);
    unsafe {
        grail_common_sort(
            arr.as_mut_ptr(),
            arr.len() as isize,
            ext_buffer.as_mut_ptr() as *mut T,
            STATIC_BUFFER_LENGTH as isize,
            &mut cmp,
        );
    }
}

#[cfg(feature = "alloc")]
pub fn grail_sort_with_dynamic_buffer<T: Ord>(arr: &mut [T]) {
    extern crate alloc;
    use alloc::vec::Vec;

    if arr.is_empty() {
        return;
    }

    let mut temp_len = 1;
    while temp_len * temp_len < arr.len() {
        temp_len *= 2;
    }

    let mut ext_buffer: Vec<core::mem::MaybeUninit<T>> = Vec::with_capacity(temp_len);
    unsafe {
        ext_buffer.set_len(temp_len);
    }

    let mut cmp = |a: &T, b: &T| a.cmp(b);
    unsafe {
        grail_common_sort(
            arr.as_mut_ptr(),
            arr.len() as isize,
            ext_buffer.as_mut_ptr() as *mut T,
            temp_len as isize,
            &mut cmp,
        );
    }
}

unsafe fn grail_rec_merge<T, F>(arr: *mut T, len1: isize, len2: isize, cmp: &mut F)
where
    F: FnMut(&T, &T) -> Ordering,
{
    if len1 < 3 || len2 < 3 {
        grail_merge_without_buffer(arr, len1, len2, cmp);
        return;
    }

    let midpoint = if len1 < len2 {
        len1 + len2 / 2
    } else {
        len1 / 2
    };

    let len1_left = grail_binary_search(arr, len1, &*arr.offset(midpoint), false, cmp);
    let mut len1_right = len1_left;

    if len1_right < len1 && cmp(&*arr.offset(len1_right), &*arr.offset(midpoint)) == Ordering::Equal
    {
        len1_right = grail_binary_search(
            arr.offset(len1_left),
            len1 - len1_left,
            &*arr.offset(midpoint),
            true,
            cmp,
        ) + len1_left;
    }

    let len2_left = grail_binary_search(arr.offset(len1), len2, &*arr.offset(midpoint), false, cmp);
    let mut len2_right = len2_left;

    if len2_right < len2
        && cmp(&*arr.offset(len1 + len2_right), &*arr.offset(midpoint)) == Ordering::Equal
    {
        len2_right = grail_binary_search(
            arr.offset(len1 + len2_left),
            len2 - len2_left,
            &*arr.offset(midpoint),
            true,
            cmp,
        ) + len2_left;
    }

    if len1_left == len1_right {
        grail_rotate(arr.offset(len1_right), len1 - len1_right, len2_right);
    } else {
        grail_rotate(arr.offset(len1_left), len1 - len1_left, len2_left);

        if len2_right != len2_left {
            grail_rotate(
                arr.offset(len1_right + len2_left),
                len1 - len1_right,
                len2_right - len2_left,
            );
        }
    }

    grail_rec_merge(
        arr.offset(len1_right + len2_right),
        len1 - len1_right,
        len2 - len2_right,
        cmp,
    );
    grail_rec_merge(arr, len1_left, len2_left, cmp);
}

pub fn rec_stable_sort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }

    let mut cmp = |a: &T, b: &T| a.cmp(b);
    unsafe {
        let ptr = arr.as_mut_ptr();
        let len = arr.len() as isize;

        let mut dist = 1;
        while dist < len {
            if cmp(&*ptr.offset(dist - 1), &*ptr.offset(dist)) == Ordering::Greater {
                ptr::swap(ptr.offset(dist - 1), ptr.offset(dist));
            }
            dist += 2;
        }

        let mut part = 2;
        while part < len {
            let mut left = 0;
            let right = len - 2 * part;

            while left <= right {
                grail_rec_merge(ptr.offset(left), part, part, &mut cmp);
                left += 2 * part;
            }

            let rest = len - left;
            if rest > part {
                grail_rec_merge(ptr.offset(left), part, rest - part, &mut cmp);
            }
            part *= 2;
        }
    }
}

use rand::seq::SliceRandom;

pub struct FirstMove;

impl FirstMove {
    pub fn find_bot_first_move_5x5(p1_mark_idx: usize) -> u8 {
        let center_idx = 12usize;

        match p1_mark_idx {
            0 | 4 | 6 | 8 | 16 | 18 | 20 | 24 => center_idx as u8,
            5 | 9 => Self::get_random_or_fallback_idx(&[p1_mark_idx + 10], p1_mark_idx + 5),
            15 | 19 => Self::get_random_or_fallback_idx(&[p1_mark_idx - 10], p1_mark_idx - 5),
            1 | 21 => Self::get_random_or_fallback_idx(&[p1_mark_idx + 2], p1_mark_idx + 1),
            3 | 23 => Self::get_random_or_fallback_idx(&[p1_mark_idx - 2], p1_mark_idx - 1),
            2 | 22 => {
                let indices = [p1_mark_idx - 1, p1_mark_idx + 1];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx - 1)
            }
            10 | 14 => {
                let indices = [p1_mark_idx - 5, p1_mark_idx + 5];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx - 5)
            }
            7 => {
                let indices = [
                    p1_mark_idx - 1,
                    p1_mark_idx + 1,
                    p1_mark_idx + 5,
                    p1_mark_idx + 10,
                ];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx + 5)
            }
            17 => {
                let indices = [
                    p1_mark_idx - 1,
                    p1_mark_idx + 1,
                    p1_mark_idx - 5,
                    p1_mark_idx - 10,
                ];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx - 5)
            }
            11 => {
                let indices = [p1_mark_idx - 5, p1_mark_idx + 1, p1_mark_idx + 5];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx + 1)
            }
            13 => {
                let indices = [p1_mark_idx - 5, p1_mark_idx - 1, p1_mark_idx + 5];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx - 1)
            }
            _ => {
                // p1_mark_idx == center_idx
                let indices = [6, 8, 16, 18];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx - 5)
            }
        }
    }

    fn get_random_or_fallback_idx(indices: &[usize], fallback: usize) -> u8 {
        if let Some(&rand_idx) = indices.choose(&mut rand::thread_rng()) {
            rand_idx as u8
        } else {
            fallback as u8
        }
    }
}

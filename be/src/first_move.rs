use rand::seq::SliceRandom;

pub struct FirstMove;

impl FirstMove {
    pub fn find_bot_first_move_5x5(p1_mark_idx: usize) -> u8 {
        let center_idx = 12usize;

        match p1_mark_idx {
            0 | 4 | 6 | 8 | 16 | 18 | 20 | 24 => center_idx as u8,
            1 | 5 => {
                let indices = [
                    p1_mark_idx + 1,
                    p1_mark_idx + 2,
                    p1_mark_idx + 5,
                    p1_mark_idx + 10,
                ];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx + 1)
            }
            3 | 9 => {
                let indices = [
                    p1_mark_idx - 2,
                    p1_mark_idx - 1,
                    p1_mark_idx + 5,
                    p1_mark_idx + 10,
                ];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx - 1)
            }
            15 | 21 => {
                let indices = [
                    p1_mark_idx - 10,
                    p1_mark_idx - 5,
                    p1_mark_idx + 1,
                    p1_mark_idx + 2,
                ];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx + 1)
            }
            19 | 23 => {
                let indices = [
                    p1_mark_idx - 10,
                    p1_mark_idx - 5,
                    p1_mark_idx - 1,
                    p1_mark_idx - 2,
                ];
                Self::get_random_or_fallback_idx(&indices, p1_mark_idx - 1)
            }
            2 => {
                let indices = [
                    p1_mark_idx - 1,
                    p1_mark_idx + 1,
                    p1_mark_idx + 5,
                    center_idx,
                ];
                Self::get_random_or_fallback_idx(&indices, center_idx)
            }
            10 => {
                let indices = [
                    p1_mark_idx - 5,
                    p1_mark_idx + 5,
                    p1_mark_idx + 1,
                    center_idx,
                ];
                Self::get_random_or_fallback_idx(&indices, center_idx)
            }
            14 => {
                let indices = [
                    p1_mark_idx - 5,
                    p1_mark_idx + 5,
                    p1_mark_idx - 1,
                    center_idx,
                ];
                Self::get_random_or_fallback_idx(&indices, center_idx)
            }
            22 => {
                let indices = [
                    p1_mark_idx - 1,
                    p1_mark_idx + 1,
                    p1_mark_idx - 5,
                    center_idx,
                ];
                Self::get_random_or_fallback_idx(&indices, center_idx)
            }
            7 | 11 | 13 | 17 => center_idx as u8,
            _ => {
                let indices = [6, 8, 16, 18];
                Self::get_random_or_fallback_idx(&indices, 6)
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

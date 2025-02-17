#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Correctness {
    /// Green
    Correct,
    /// Yellow
    Misplaced,
    /// Gray
    Incorrect,
}

impl Correctness {

    pub fn check(answer: &str, guess: &str) -> [Self; 5] {
        let mut result = [Correctness::Incorrect; 5];
        let answer_bytes = answer.as_bytes();
        let guess_bytes = guess.as_bytes();
        let mut misplaced = [0u8; (b'z' - b'a' + 1) as usize];
        for ((a, g), r) in answer_bytes.iter().zip(guess_bytes).zip(result.iter_mut()) {
            if a == g {
                *r = Correctness::Correct;
            } else {
                misplaced[(a-b'a') as usize] += 1;
            }
        }

        for (g, r) in guess_bytes.iter().zip(result.iter_mut()) {
            if *r == Correctness::Incorrect && misplaced[(g-b'a') as usize] > 0 {
                *r = Correctness::Misplaced;
                misplaced[(g-b'a') as usize] -= 1;
            }
        }
        result
    }

}

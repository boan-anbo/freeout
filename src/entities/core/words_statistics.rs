use serde::{Deserialize, Serialize};
use words_count::{count, WordsCount};

/// # Word Stats
#[derive(Serialize, Deserialize, Clone)]
pub struct WordCount {
    pub words: usize,
    pub characters: usize,
}

// default
impl Default for WordCount {
    fn default() -> Self {
        Self {
            words: 0,
            characters: 0,
        }
    }
}

impl From<words_count::WordsCount> for WordCount {
    fn from(words_count: words_count::WordsCount) -> Self {
        Self {
            words: words_count.words,
            characters: words_count.characters,
        }
    }
}

/// # Word Expectation
#[derive(Serialize, Deserialize, Clone)]
pub struct WordsTarget {
    /// The expected number of words.
    pub words: usize,
    /// Distribution method (uniform, custom, etc.)
    pub distribution: Option<DistributionMethod>,
}

// default
impl Default for WordsTarget {
    fn default() -> Self {
        Self {
            words: 0,
            distribution: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum DistributionMethod {
    Uniform, // each sub-block gets the same share of words
             // ... any other methods you might want to introduce
}


/// # Word Reality
///
/// Computed entity to represents the overflow or
#[derive(Serialize, Deserialize, Clone)]
pub struct WordsStatus {
    /// The actual balance of words. A words goal of 100 and a words reality of 120 means 20.
    pub balance: isize,
    /// The "real expectation" is essentially a dynamic computation that takes into account the actual words used by preceding siblings and adjusts the expectation for the subsequent siblings accordingly.
    /// This scenario can emerge if, for instance, you have a parent block with a WordsGoal of 500 words, and it has three children. Ideally, with a uniform distribution, each child should have a goal of approximately 167 words.
    /// But if the first two children have already consumed 400 words combined, then the "real expectation" for the third child would be 100 words.
    pub adjusted_target: Option<usize>,
}

// default
impl Default for WordsStatus {
    fn default() -> Self {
        Self {
            balance: 0,
            adjusted_target: None,
        }
    }
}

// A representation of word statistics.
#[derive(Serialize, Deserialize, Clone)]
pub struct WordStatistics {
    pub target: Option<WordsTarget>,
    pub status: Option<WordsStatus>,
    pub count: WordCount,
}

// Default
impl Default for WordStatistics {
    fn default() -> Self {
        Self {
            target: None,
            status: None,
            count: WordCount::default(),
        }
    }
}

impl WordStatistics {
    /// # Count words in a string
    pub fn count(&mut self, text: &str)  {
        let word_count = count(text);
        self.count = WordCount::from(word_count)
    }
}

impl WordStatistics {
    pub fn new(word_count: WordCount) -> Self {
        Self {
            target: None,
            count: word_count,
            status: None,
        }
    }

    pub fn update(&mut self) {
       todo!()
    }

    pub fn calculate(text: &str) -> Self {
        todo!()
    }
}

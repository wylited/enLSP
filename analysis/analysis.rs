

// /// Enumeration of all the parts of speech.
// /// Any word can be one, or more of these.
// /// The parts of speech are used to determine the meaning of a word as well as the context of the word.
// pub enum SpeechType {
//     NOUN,
//     PRONOUN,
//     VERB,
//     ADJECTIVE,
//     ADVERB,
//     PREPOSITION,
//     CONJUCTION,
//     INTERJECTION,
//     ARTICLE,
// }

// /// Enumeration of all the tenses I want to support.
// /// The tenses are used to determine the time context of the word as well as the meaning context of the word.
// pub enum TenseType {
//     SPRESENT,   // Simple Present
//     SPAST,      // Simple Fast
//     SFUTURE,    // Simple Future
//     CPRESENT,   // Continuous Present
//     CPAST,      // Continuous Past
//     CFUTURE,    // Continuous Future
//     PPRESENT,   // Perfect Present
//     PPAST,      // Perfect Past
//     PFUTURE,    // Perfect Future
//     PCPRESENT,  // Perfect Continuous Present
//     PCPAST,     // Perfect Continuous Past
//     PCFUTURE,   // Perfect Continuous Future
//     INFINITIVE  // Infinitive
// }

// pub struct Quote {
//     pub text: Vec<Word>
// }

// pub struct Word {
//     pub word: Vec<char>, // The word itself
//     pub global_pos: (u32, u32), // The global position of a start of a word. (line, column)
//     pub word_index: usize, // The global index of the word.
//     pub relative_pos: usize, // The relative position of a word. (relative to the sentence) 
//     pub speech: Option<SpeechType>, // The type of the word. If None, the word cannot be parsed.
//     pub tense: Option<TenseType>,  // The tense of the word. If None, the word cannot be parsed.
//     pub definition: Option<String>, // The definition of the word, if it can be matched. Otherwise it is None
// }

// pub struct Sentence {
//     pub words: Vec<Word>,
//     pub global_pos: (u32, u32), // The global position of a start of a sentence. (line, column)
//     pub sentence_index: usize, // The global index of the sentence.
//     pub relative_pos: usize, // The relative position of a sentence. (relative to the document)
// }


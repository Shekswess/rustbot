use super::{Form, Posting, intersects};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub(super) struct Span {
    start: u32,
    end: u32,
}

impl Span {
    pub(super) fn added<T>(items: &[T], start: usize) -> Self {
        Self {
            start: u32::try_from(start).expect("knowledge index offset overflow"),
            end: u32::try_from(items.len()).expect("knowledge index length overflow"),
        }
    }

    pub(super) fn slice<T>(self, items: &[T]) -> &[T] {
        &items[self.start as usize..self.end as usize]
    }
}

pub(super) type Word = u32;

#[derive(Clone, Copy)]
pub(super) struct StoredPhrase {
    pub(super) words: Span,
    pub(super) unique_in_domain: u32,
}

/// Contiguous compiled forms, word sequences, alternatives, and key groups.
#[derive(Default)]
pub(super) struct Arena {
    pub(super) forms: Vec<Form>,
    pub(super) words: Vec<Span>,
    pub(super) phrase_words: Vec<Word>,
    pub(super) phrases: Vec<StoredPhrase>,
    pub(super) groups: Vec<Span>,
}

impl Arena {
    pub(super) fn shrink_to_fit(&mut self) {
        self.forms.shrink_to_fit();
        self.words.shrink_to_fit();
        self.phrase_words.shrink_to_fit();
        self.phrases.shrink_to_fit();
        self.groups.shrink_to_fit();
    }

    pub(super) fn word(&self, word: Word) -> &[Form] {
        self.words[word as usize].slice(&self.forms)
    }

    pub(super) fn phrase(&self, phrase: StoredPhrase) -> Phrase<'_> {
        Phrase {
            arena: self,
            words: phrase.words.slice(&self.phrase_words),
            unique_in_domain: phrase.unique_in_domain as usize,
        }
    }

    pub(super) fn group(&self, span: Span) -> Group<'_> {
        Group {
            arena: self,
            phrases: span.slice(&self.phrases),
        }
    }

    pub(super) fn key(&self, span: Span) -> Key<'_> {
        Key {
            arena: self,
            groups: span.slice(&self.groups),
        }
    }
}

#[derive(Clone, Copy)]
pub(super) struct Key<'a> {
    arena: &'a Arena,
    groups: &'a [Span],
}

impl<'a> Key<'a> {
    pub(super) fn len(self) -> usize {
        self.groups.len()
    }

    pub(super) fn iter(self) -> impl ExactSizeIterator<Item = Group<'a>> {
        self.groups.iter().map(move |&span| self.arena.group(span))
    }

    pub(super) fn first(self) -> Option<Group<'a>> {
        self.iter().next()
    }
}

#[derive(Clone, Copy)]
pub(super) struct Group<'a> {
    arena: &'a Arena,
    phrases: &'a [StoredPhrase],
}

impl<'a> Group<'a> {
    pub(super) fn len(self) -> usize {
        self.phrases.len()
    }

    pub(super) fn is_empty(self) -> bool {
        self.phrases.is_empty()
    }

    pub(super) fn iter(self) -> impl ExactSizeIterator<Item = Phrase<'a>> {
        self.phrases
            .iter()
            .map(move |&phrase| self.arena.phrase(phrase))
    }
}

#[derive(Clone, Copy)]
pub(super) struct Phrase<'a> {
    arena: &'a Arena,
    words: &'a [Word],
    pub(super) unique_in_domain: usize,
}

impl<'a> Phrase<'a> {
    pub(super) fn len(self) -> usize {
        self.words.len()
    }

    pub(super) fn word(self, index: usize) -> Option<&'a [Form]> {
        self.words.get(index).map(|&word| self.arena.word(word))
    }

    pub(super) fn words(self) -> impl DoubleEndedIterator<Item = &'a [Form]> + ExactSizeIterator {
        self.words.iter().map(move |&word| self.arena.word(word))
    }

    pub(super) fn matches<F: AsRef<[Form]>>(self, line: &[F]) -> bool {
        !self.words.is_empty()
            && self.len() == line.len()
            && self
                .words()
                .zip(line)
                .all(|(key, word)| intersects(key, word.as_ref()))
    }

    pub(super) fn spans<'line, F: AsRef<[Form]> + 'line>(
        self,
        line: &'line [F],
    ) -> impl Iterator<Item = (usize, usize)> + 'line
    where
        'a: 'line,
    {
        let len = self.len().max(1);
        line.windows(len)
            .enumerate()
            .filter(move |(_, span)| self.matches(span))
            .map(move |(start, _)| (start, start + len))
    }

    pub(super) fn occurs(self, line: &[Vec<Form>]) -> bool {
        self.spans(line).next().is_some()
    }
}

#[derive(Default)]
pub(super) struct PostingIndex {
    spans: Box<[Span]>,
    postings: Box<[Posting]>,
}

impl PostingIndex {
    pub(super) fn pack(mut pairs: Vec<(Form, Posting)>, form_count: usize) -> Self {
        let mut postings = Vec::with_capacity(pairs.len());
        // A posting occurs once per form and row. Keep the registration order
        // within a bucket while packing the buckets by their dense form IDs.
        pairs.sort_unstable_by_key(|&(form, posting)| (form, posting.index()));
        let mut pairs = pairs.into_iter().peekable();
        let spans = (0..form_count)
            .map(|index| {
                let start = postings.len();
                let form = Form::try_from(index).expect("knowledge form index overflow");
                while let Some((_, posting)) = pairs.next_if(|&(pair_form, _)| pair_form == form) {
                    postings.push(posting);
                }
                Span::added(&postings, start)
            })
            .collect();
        Self {
            spans,
            postings: postings.into_boxed_slice(),
        }
    }

    pub(super) fn get(&self, form: Form) -> &[Posting] {
        self.spans
            .get(form as usize)
            .map_or(&[], |span| span.slice(&self.postings))
    }
}

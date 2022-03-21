#![allow(dead_code)]

use std::iter;

#[derive(Clone)]
pub struct AnimeCollection(Vec<Anime>);

impl AnimeCollection {
    fn new() -> AnimeCollection {
        AnimeCollection(Vec::new())
    }

    fn add(&mut self, elem: Anime) {
        self.0.push(elem);
    }

    pub fn get_elem(&self, index: usize) -> Anime {
        self.0[index].clone()
    }

    pub fn print_ele(&self) {
        let mut i = 1;

        for anime in self.0.iter() {
            println!("{}. {}", i, anime.title);
            i += 1;
        }
    }
}

impl iter::Iterator for AnimeCollection {
    type Item = Anime;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl iter::FromIterator<Anime> for AnimeCollection {
    fn from_iter<I: IntoIterator<Item = Anime>>(iter: I) -> Self {
        let mut c = AnimeCollection::new();

        for i in iter {
            c.add(i);
        }

        c
    }
}

#[derive(Clone, Debug, Default)]
pub struct Anime {
    pub title: String,
    pub url: String,
    pub episodes: Vec<u32>,
}

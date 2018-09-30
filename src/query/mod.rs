/*!
Query
*/

mod query;
mod boolean_query;
mod scorer;
mod occur;
mod weight;
mod term_query;
mod query_parser;
mod phrase_query;
mod all_query;
mod bitset;
mod exclude;
mod union;
mod intersection;
mod reqopt_scorer;
mod bm25;

#[cfg(test)]
mod vec_docset;

pub(crate) mod score_combiner;

pub use self::intersection::Intersection;
pub use self::union::Union;

#[cfg(test)]
pub use self::vec_docset::VecDocSet;

pub use self::reqopt_scorer::RequiredOptionalScorer;
pub use self::exclude::Exclude;
pub use self::bitset::BitSetDocSet;
pub use self::boolean_query::BooleanQuery;
pub use self::occur::Occur;
pub use self::phrase_query::PhraseQuery;
pub use self::query_parser::QueryParserError;
pub use self::query_parser::QueryParser;
pub use self::query::Query;
pub use self::scorer::EmptyScorer;
pub use self::scorer::Scorer;
pub use self::term_query::TermQuery;
pub use self::weight::Weight;
pub use self::all_query::{AllQuery, AllScorer, AllWeight};
pub use self::scorer::ConstScorer;
pub use self::intersection::intersect_scorers;

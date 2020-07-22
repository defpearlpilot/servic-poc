// use cdrs::frame::traits::TryFromRow;
use cdrs::query::QueryValues;
use cdrs::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Dot {
    pub dot_id: i32,
    pub author: String,
    pub subject: String,
    pub attribute: String,
    pub comment: String,
    pub rating: i32,
    pub importance: i32,
}

impl Dot {
    pub fn into_query_values(self) -> QueryValues {
        query_values!(
        "dot_id" => self.dot_id, 
        "author" => self.author, 
        "subject" => self.subject,
        "attribute" => self.attribute, 
        "comment" => self.comment, 
        "rating" => self.rating, 
        "importance" => self.importance)
    }

    pub fn new(
        dot_id: i32,
        author: String,
        subject: String,
        attribute: String,
        comment: String,
        rating: i32,
        importance: i32,
    ) -> Self {
        Dot {
            dot_id,
            author,
            subject,
            attribute,
            comment,
            rating,
            importance,
        }
    }
}

use boolinator::Boolinator;
use chrono::NaiveDate;
use diesel::{prelude::*, dsl::count};
use serde::{
    Serialize,
    Deserialize
};
use uuid::Uuid;

use crate::schema::*;

use super::{error::{db_map_err, DbError}, IdentifiableWithUuid};

#[derive(Queryable, Insertable, Selectable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = questions)]
pub struct Question {
    id: String,
    pub label: String,
    pub topic: String
}

impl IdentifiableWithUuid for Question {
    fn id(&self) -> Uuid {
        Uuid::parse_str(&self.id)
            .expect("Corrupted Uuid! Cannot proceed.")
    }
}

impl Question {
    pub fn new(label: &str, topic: &str) -> Self {
        Self { 
            id: Uuid::new_v4().to_string(),
            label: label.to_string(),
            topic: topic.to_string()
        }
    }

    pub fn insert(self, conn: &mut SqliteConnection) -> Result<(), DbError> {
        diesel::insert_into(questions::table)
            .values(self)
            .execute(conn)
            .map_err(db_map_err)?;

        Ok(())
    }

    pub fn select_topics(conn: &mut SqliteConnection) 
    -> Result<Vec<String>, DbError> {
        questions::table
            .select(questions::topic)
            .distinct()
            .load(conn)
            .map_err(db_map_err)
    }

    pub fn select_question_for_topic(topic: &str, conn: &mut SqliteConnection) 
    -> Result<Vec<Question>, DbError> {
        questions::table
            .filter(questions::topic.eq(topic))
            .load(conn)
            .map_err(db_map_err)
    }
      
    pub fn select_all(conn: &mut SqliteConnection) 
    -> Result<Vec<Question>, DbError> {
        questions::table
            .load(conn)
            .map_err(db_map_err)
    }

    pub fn delete(&self, conn: &mut SqliteConnection)
    -> Result<(), DbError> {
        diesel::delete(questions::table.find(self.id().to_string()))
            .execute(conn)
            .map_err(db_map_err)?;

        Ok(())
    }

}

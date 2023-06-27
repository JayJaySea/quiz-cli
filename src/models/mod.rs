use uuid::Uuid;

pub mod question;
pub mod error;
pub mod answer;

pub trait IdentifiableWithUuid {
    fn id(&self) -> Uuid;
}

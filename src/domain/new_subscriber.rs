use crate::domain::email::Email;
use crate::domain::subscriber_name::SubscriberName;

pub struct NewSubscriber {
    pub email: Email,
    pub name: SubscriberName,
}
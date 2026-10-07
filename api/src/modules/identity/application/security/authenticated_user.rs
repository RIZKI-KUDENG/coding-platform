use uuid::Uuid;

pub struct AuthenticatedUser{
    pub user_id: Uuid,
}

pub struct AdminUser{
    pub user_id: Uuid,
}

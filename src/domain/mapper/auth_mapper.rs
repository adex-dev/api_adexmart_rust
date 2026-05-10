use crate::domain::store_entity::{LoginStore, StoreEntity};
use crate::domain::user::{ UserAccessEntity, UserAccessResponse, UserEntity, UserResponse};

impl From<UserEntity> for UserResponse {
    fn from(user: UserEntity) -> Self {
        Self {
            id: user.id,
            id_user: user.id_user,
            firstname: user.firstname,
            lastname: user.lastname,
            fullname: user.fullname,
            username: user.username,
            roles: user.roles,
            status: user.status,
            store_id: user.store_id,
            created_at: user.created_at,
            modifed_at: user.modifed_at,
        }
    }
}
impl From<UserAccessEntity> for UserAccessResponse {
    fn from(m: UserAccessEntity) -> Self {
        Self {
            id: m.id,
            name: m.name,
        }
    }
}
impl From<StoreEntity> for LoginStore {
    fn from(s: StoreEntity) -> Self {
        Self {
            store_id: s.store_id,
            name: s.name,
            address: s.address,
            cities: s.cities,
            province: s.province,
            phone1: s.phone1,
            phone2: s.phone2,
            parent_area: s.parent_area,
        }
    }
}
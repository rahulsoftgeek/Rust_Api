use super::schema::employees;

#[derive(serde::Serialize, serde::Deserialize, Queryable, AsChangeset)]
pub struct Employee {
    pub id: i32,
    pub name: String,
    pub email: String,
    #[serde(skip_deserializing)]
    pub created_at: String,
}

#[derive(Insertable, serde::Deserialize)]
#[table_name="employees"]
pub struct NewEmployee {
    pub name: String,
    pub email: String,
}

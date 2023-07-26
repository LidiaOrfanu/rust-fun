
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct Penguin {
    pub id: i32,
    pub name: String,
    pub species: String,
    pub age: i32
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct NewPenguin {
    pub name: String,
}
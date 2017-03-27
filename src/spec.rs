#[derive(Deserialize, Debug)]
pub struct FieldSpec {
    pub name: String,
    pub aggregate: String,
}

#[derive(Deserialize, Debug)]
pub struct ShSpec {
    pub fields: Vec<FieldSpec>,
    pub ecs_core_namespace: String,
    pub ecs_content_namespace: String,
    pub position: String,
}

use handlebars::Handlebars;

use spec::ShSpec;
use templates;

#[derive(Serialize, Debug, Clone)]
struct FieldTemplateData {
    pub name: String,
    pub aggregate: String,
    #[serde(rename = "type")]
    pub type_name: String,
    pub iter: bool,
    pub sum_f64: bool,
    pub count_bool: bool,
    pub any: bool,
    pub set: bool,
    pub void: bool,
    pub setlike: bool,
}

#[derive(Serialize, Debug)]
struct TemplateData {
    pub fields: Vec<FieldTemplateData>,
    pub update_fields: Vec<FieldTemplateData>,
    pub ecs_core_namespace: String,
    pub ecs_content_namespace: String,
    pub position: String,
}

fn aggregate_type_name(aggregate: &str) -> Option<String> {
    match aggregate {
        "sum_f64" => Some("f64".to_string()),
        "count_bool" => Some("usize".to_string()),
        "any" => Some("EntitySet".to_string()),
        "set" => Some("EntitySet".to_string()),
        "void" => Some("void".to_string()),
        _ => None,
    }
}

fn aggregate_is_iter(aggregate: &str) -> bool {
    match aggregate {
        "any" => true,
        "set" => true,
        _ => false,
    }
}

impl TemplateData {
    fn new(spec: &ShSpec) -> Self {
        let mut data = TemplateData {
            fields: Vec::new(),
            update_fields: Vec::new(),
            ecs_core_namespace: spec.ecs_core_namespace.clone(),
            ecs_content_namespace: spec.ecs_content_namespace.clone(),
            position: spec.position.clone(),
        };

        for c in spec.fields.iter() {
            let type_name = aggregate_type_name(c.aggregate.as_ref())
                .expect("Unknown agregate");

            let field_data = FieldTemplateData {
                name: c.name.clone(),
                aggregate: c.aggregate.clone(),
                type_name: type_name,
                iter: aggregate_is_iter(c.aggregate.as_ref()),
                sum_f64: c.aggregate == "sum_f64",
                count_bool: c.aggregate == "count_bool",
                any: c.aggregate == "any",
                set: c.aggregate == "set",
                void: c.aggregate == "void",
                setlike: c.aggregate == "any" || c.aggregate == "set",
            };

            if field_data.aggregate != "void" {
                data.fields.push(field_data.clone());
            }

            data.update_fields.push(field_data);
        }

        data
    }
}

pub fn full_template() -> String {
    templates::HEADER.to_string() +
        templates::COORD +
        templates::SPATIAL_HASH_CELL +
        templates::SPATIAL_HASH_TABLE
}

pub fn render(spec: &ShSpec) -> String {

    let data = TemplateData::new(spec);

    let mut handlebars = Handlebars::new();

    // prevent xml escaping
    handlebars.register_escape_fn(|input| input.to_string());
    handlebars.template_render(&full_template(), &data)
        .expect("Failed to render template")
}

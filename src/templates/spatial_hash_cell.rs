pub const SPATIAL_HASH_CELL: &'static str = r#"

#[derive(Serialize, Deserialize)]
pub struct SpatialHashCell {
{{#each fields}}
    {{name}}: {{type}},
{{/each}}
    _entities: EntitySet,
    _last_updated: u64,
}

fn new_field_sum_f64() -> f64 { 0.0 }
fn new_field_count_bool() -> usize { 0 }
fn new_field_any() -> EntitySet { EntitySet::new() }
fn new_field_set() -> EntitySet { EntitySet::new() }

macro_rules! bool_from_sum_f64 {
    ( $x:expr ) => { $x != 0.0 }
}
macro_rules! bool_from_count_bool {
    ( $x:expr ) => { $x != 0 }
}
macro_rules! bool_from_any {
    ( $x:expr ) => { !$x.is_empty() }
}
macro_rules! bool_from_set {
    ( $x:expr ) => { !$x.is_empty() }
}

impl SpatialHashCell {
    fn new() -> Self {
        SpatialHashCell {
{{#each fields}}
            {{name}}: new_field_{{aggregate}}(),
{{/each}}
            _entities: EntitySet::new(),
            _last_updated: 0,
        }
    }

    pub fn last_updated(&self) -> u64 { self._last_updated }
    pub fn entity_ids(&self) -> &EntitySet { &self._entities }
    pub fn entity_id_iter(&self) -> EntitySetIter { self._entities.iter() }

{{#each fields}}
    pub fn is_{{name}}(&self) -> bool {
        bool_from_{{aggregate}}!(self.{{name}})
    }
    pub fn has_{{name}}(&self) -> bool { self.is_{{name}}() }
    {{#if iter}}
    pub fn iter_{{name}}(&self) -> EntitySetIter {
        self.{{name}}.iter()
    }
    {{/if}}
    {{#if any}}
    pub fn any_{{name}}(&self) -> Option<EntityId> {
        self.{{name}}.iter().next()
    }
    {{/if}}
    fn remove_{{name}}(&mut self, entity: EntityRef) {
        {{#if sum_f64}}
        if let Some(v) = entity.copy_{{name}}() {
            self.{{name}} -= v;
        }
        {{/if}}
        {{#if count_bool}}
        if entity.contains_{{name}}() {
            self.{{name}} -= 1;
        }
        {{/if}}
        {{#if setlike}}
        if entity.contains_{{name}}() {
            self.{{name}}.remove(entity.id());
        }
        {{/if}}
    }
    fn insert_{{name}}(&mut self, entity: EntityRef) {
        {{#if sum_f64}}
        if let Some(v) = entity.copy_{{name}}() {
            self.{{name}} += v;
        }
        {{/if}}
        {{#if count_bool}}
        if entity.contains_{{name}}() {
            self.{{name}} += 1;
        }
        {{/if}}
        {{#if setlike}}
        if entity.contains_{{name}}() {
            self.{{name}}.insert(entity.id());
        }
        {{/if}}
    }
    {{#if sum_f64}}
    pub fn get_{{name}}(&self) -> f64 {
        self.{{name}}
    }
    {{/if}}
{{/each}}
}
"#;

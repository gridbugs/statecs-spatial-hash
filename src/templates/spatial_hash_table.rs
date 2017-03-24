pub const SPATIAL_HASH_TABLE: &'static str = r#"

pub type SpatialHashTableIter<'a> = slice::Iter<'a, SpatialHashCell>;
pub struct SpatialHashTableCoordIter {
    len: usize,
    width: usize,
    idx: usize,
}

impl Iterator for SpatialHashTableCoordIter {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {

        if self.idx == self.len {
            return None;
        }

        let x = self.idx % self.width;
        let y = self.idx / self.width;

        self.idx += 1;

        Some((x, y))
    }
}

#[derive(Serialize, Deserialize)]
pub struct SpatialHashTable {
    width: usize,
    height: usize,
    grid: Vec<SpatialHashCell>,
    empty: SpatialHashCell,
}

impl SpatialHashTable {
    pub fn new(width: usize, height: usize) -> Self {
        let capacity = width * height;
        SpatialHashTable {
            width: width,
            height: height,
            grid: {
                let mut v = Vec::with_capacity(capacity);

                for _ in 0..capacity {
                    v.push(SpatialHashCell::new());
                }

                v
            },
            empty: SpatialHashCell::new(),
        }
    }

    pub fn iter(&self) -> SpatialHashTableIter {
        self.grid.iter()
    }

    pub fn coord_iter(&self) -> SpatialHashTableCoordIter {
        SpatialHashTableCoordIter {
            len: self.grid.len(),
            width: self.width,
            idx: 0,
        }
    }

    pub fn clear(&mut self) {
        self.grid.clear()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn is_valid_coord<C: SpatialHashTableCoord>(&self, c: C) -> bool {
        c.x() < self.width() && c.y() < self.height()
    }

    pub fn is_valid_signed_coord(&self, x: isize, y: isize) -> bool {
        x >= 0 && x < self.width() as isize && y >= 0 && y < self.height() as isize
    }

    pub fn limits_min(&self) -> (isize, isize) {
        (0, 0)
    }

    pub fn limits_max(&self) -> (isize, isize) {
        (self.width() as isize - 1, self.height() as isize - 1)
    }

    pub fn get<C: SpatialHashTableCoord>(&self, c: C) -> &SpatialHashCell {
        self.grid.get(c.to_linear_index(self.width)).unwrap_or(&self.empty)
    }

    fn get_mut<C: SpatialHashTableCoord>(&mut self, c: C) -> &mut SpatialHashCell {
        unsafe {
            self.grid.get_unchecked_mut(c.to_linear_index(self.width))
        }
    }

    fn change_entity_position<C: SpatialHashTableCoord>(
        &mut self, entity: EntityRef,
        current_position: C, new_position: C,
        action_id: u64) {

        self.remove_entity_position(entity, current_position, action_id);
        self.add_entity_position(entity, new_position, action_id);

    }

    fn remove_entity_position<C: SpatialHashTableCoord>(
        &mut self, entity: EntityRef,
        position: C, action_id: u64) {

        let mut cell = self.get_mut(position);

{{#each fields}}
        cell.remove_{{name}}(entity);
{{/each}}

        cell._entities.remove(entity.id());
        cell._last_updated = action_id;
    }

    fn add_entity_position<C: SpatialHashTableCoord>(
        &mut self, entity: EntityRef,
        position: C, action_id: u64) {

        let mut cell = self.get_mut(position);

{{#each fields}}
        cell.insert_{{name}}(entity);
{{/each}}

        cell._entities.insert(entity.id());
        cell._last_updated = action_id;

    }

    pub fn update(&mut self, ecs: &EcsCtx, action: &EcsAction, action_id: u64) {

        for (entity_id, new_position) in action.positive_copy_iter_{{position}}(ecs) {
            let entity = ecs.entity(entity_id);
            // Add and remove tracked components based on the current data stored about the
            // entity, ignoring any component changes in the current action. These will be
            // applied later.
            if let Some(current_position) = entity.copy_{{position}}() {
                // the entity is changing position
                self.change_entity_position(entity, current_position, new_position, action_id);
            } else {
                // the entity is gaining a position
                self.add_entity_position(entity, new_position, action_id);
            }
        }

        for entity_id in action.negative_iter_{{position}}(ecs) {
            let entity = ecs.entity(entity_id);
            if let Some(position) = entity.copy_{{position}}() {
                self.remove_entity_position(entity, position, action_id);
            }
        }

{{#each update_fields}}
        self.update_{{ name }}(ecs, action, action_id);
{{/each}}
    }

{{#each update_fields}}
    fn update_{{ name }}(&mut self, ecs: &EcsCtx, action: &EcsAction, action_id: u64) {
    {{#if sum_f64}}
        for (entity_id, new) in action.positive_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                let current = entity.current_copy_{{ name }}().unwrap_or(0.0);
                let increase = new - current;
                let cell = self.get_mut(position);
                cell.{{ name }} += increase;
                cell._last_updated = action_id;
            }
        }
        for entity_id in action.negative_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                if let Some(value) = entity.current_copy_{{ name }}() {
                    let cell = self.get_mut(position);
                    cell.{{ name }} -= value;
                    cell._last_updated = action_id;
                }
            }
        }
    {{/if}}
    {{#if count_bool}}
        for entity_id in action.positive_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                if !entity.current_contains_{{ name }}() {
                    let cell = self.get_mut(position);
                    cell.{{ name }} += 1;
                    cell._last_updated = action_id;
                }
            }
        }
        for entity_id in action.negative_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                if entity.current_contains_{{ name }}() {
                    let cell = self.get_mut(position);
                    cell.{{ name }} -= 1;
                    cell._last_updated = action_id;
                }
            }
        }
    {{/if}}
    {{#if setlike}}
        for entity_id in action.positive_id_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                let cell = self.get_mut(position);
                if !entity.current_contains_{{ name }}() {
                    cell.{{ name }}.insert(entity_id);
                }
                cell._last_updated = action_id;
            }
        }
        for entity_id in action.negative_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                if entity.current_contains_{{ name }}() {
                    let cell = self.get_mut(position);
                    cell.{{ name }}.remove(entity_id);
                    cell._last_updated = action_id;
                }
            }
        }
    {{/if}}
    {{#if void}}
        for entity_id in action.positive_id_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                self.get_mut(position)._last_updated = action_id;
            }
        }

        for entity_id in action.negative_iter_{{ name }}(ecs) {
            let entity = ecs.post_entity(action, entity_id);
            if let Some(position) = entity.copy_{{../position}}() {
                self.get_mut(position)._last_updated = action_id;
            }
        }
    {{/if}}
    }
{{/each}}
}
"#;

pub const HEADER: &'static str = r#"

use std::slice;
use {{ecs_namespace}}::*;

{{#each imports}}
use {{ this }};
{{/each}}
"#;

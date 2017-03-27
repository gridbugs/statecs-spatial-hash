pub const HEADER: &'static str = r#"

use std::slice;
use {{ecs_core_namespace}}::*;
use {{ecs_content_namespace}}::*;

{{#each imports}}
use {{ this }};
{{/each}}
"#;

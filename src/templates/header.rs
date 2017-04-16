pub const HEADER: &'static str = r#"// Automatically generated. Do not edit.
#![allow(dead_code)]

use std::slice;
use {{ecs_core_namespace}}::*;
use {{ecs_content_namespace}}::*;

{{#each imports}}
use {{ this }};
{{/each}}
"#;

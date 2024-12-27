use std::path::Path;

use rbx_dom_weak::{
    types::{Attributes, Enum, Variant},
    InstanceBuilder, WeakDom,
};
use uuid::Uuid;

use crate::{roblox, Result};

const PLUGIN_BASE: &str = include_str!("plugin-base.luau");
const PORT_ATTRIBUTE: &str = "port";
const ID_ATTRIBUTE: &str = "server-id";

pub fn bundle_plugin(path_to_script: &Path, port: u16, id: Uuid) -> Result<()> {
    let mut attributes = Attributes::new();
    attributes.insert(PORT_ATTRIBUTE.to_owned(), Variant::Float64(port as f64));
    attributes.insert(
        ID_ATTRIBUTE.to_string(),
        Variant::BinaryString(id.to_string().into_bytes().into()),
    );

    let script_contents = fs_err::read_to_string(path_to_script)?;

    let root = InstanceBuilder::new("Script")
        .with_name("run-in-roblox")
        .with_properties([
            ("Source", Variant::from(PLUGIN_BASE.to_owned())),
            ("Attributes", Variant::from(attributes)),
            // ("RunContext", Variant::Enum(Enum::from_u32(3))),
        ])
        .with_child(
            InstanceBuilder::new("ModuleScript")
                .with_name("package")
                .with_property("Source", script_contents),
        );

    let dom = WeakDom::new(root);
    let mut vector = Vec::new();
    rbx_binary::to_writer(&mut vector, &dom, &[dom.root_ref()])?;

    roblox::install_plugin(&vector)
}

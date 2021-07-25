use super::{Application, RPCs, Storages};

/// Module.
pub trait Module {
    // type Storages: Storages;

    // type Events: Events;

    /// This module provided RPCs.
    type RPCs: RPCs;

    /// This module provided Application.
    type Application: Application;

    /// Get module metadata.
    fn metadata(&self) -> ModuleMetadata;

    /// Return application instance.
    fn application(&self) -> Self::Application;

    /// Return rpcs instance.
    fn rpcs(&self) -> Self::RPCs;
}

/// Metadata of module.
pub struct ModuleMetadata<'a> {
    /// Name of module.
    pub name: &'a str,
    /// Version of module. If this version change, means module need update.
    pub version: &'a str,
    /// Version of impl. If this version change, means module only a change of impl.
    pub impl_version: &'a str,
}

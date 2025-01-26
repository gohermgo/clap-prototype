use crate::plugin::str_types::*;

use clap_sys::plugin::clap_plugin_descriptor;
use clap_sys::version::clap_version;

pub struct PluginDescriptor<'desc> {
    pub framework_version: clap_version,
    pub id: &'desc PluginID,
    pub name: &'desc PluginName,
    pub vendor: &'desc PluginVendor,
    pub url: &'desc PluginURL,
    pub manual_url: &'desc PluginURL,
    pub support_url: &'desc PluginURL,
    pub version: &'desc PluginVersion,
    pub description: &'desc PluginDescription,
    pub features: RawPluginFeatureSet,
}
impl<'desc> PluginDescriptor<'desc> {
    pub fn from_raw(raw: &'desc clap_plugin_descriptor) -> PluginDescriptor<'desc> {
        println!("CONSTRUCTING PLUGIN_DESCRIPTOR {raw:#?}");
        Self {
            framework_version: raw.clap_version,
            id: unsafe { PluginID::from_ptr(raw.id) },
            name: unsafe { PluginName::from_ptr(raw.name) },
            vendor: unsafe { PluginVendor::from_ptr(raw.vendor) },
            url: unsafe { PluginURL::from_ptr(raw.url) },
            manual_url: unsafe { PluginURL::from_ptr(raw.manual_url) },
            support_url: unsafe { PluginURL::from_ptr(raw.support_url) },
            version: unsafe { PluginVersion::from_ptr(raw.version) },
            description: unsafe { PluginDescription::from_ptr(raw.version) },
            features: RawPluginFeatureSet::from_ptr(raw.features)
                .expect("failure to parse features"),
        }
    }
    pub fn into_raw(self) -> clap_plugin_descriptor {
        let raw = clap_plugin_descriptor {
            clap_version: self.framework_version,
            id: self.id.as_ptr(),
            name: self.name.as_ptr(),
            vendor: self.vendor.as_ptr(),
            url: self.url.as_ptr(),
            manual_url: self.manual_url.as_ptr(),
            support_url: self.support_url.as_ptr(),
            version: self.version.as_ptr(),
            description: self.description.as_ptr(),
            features: self.features.as_ptr(),
        };
        println!("RAW DESCRIPTOR: {raw:#?}");
        raw
    }
}
impl From<PluginDescriptor<'_>> for clap_plugin_descriptor {
    fn from(value: PluginDescriptor<'_>) -> Self {
        value.into_raw()
    }
}

use gst::glib;
use gst::prelude::*;
mod imp;

glib::wrapper! {
    pub struct BandwidthEstimator(ObjectSubclass<imp::BandwidthEstimator>) @extends gst::Element, gst::Object;
}

// // Plugin entry point that should register all elements provided by this plugin,
// // and everything else that this plugin might provide (e.g. typefinders or device providers).
pub fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    #[cfg(feature = "doc")]
    {
        imp::Estimator::static_type().mark_as_plugin_api(gst::PluginAPIFlags::empty());
    }

    gst::Element::register(
        Some(plugin),
        "rtppace",
        gst::Rank::NONE,
        BandwidthEstimator::static_type(),
    )?;

    Ok(())
}

// Static plugin metadata that is directly stored in the plugin shared object and read by GStreamer
// upon loading.
// Plugin name, plugin description, plugin entry point function, version number of this plugin,
// license of the plugin, source package name, binary package name, origin where it comes from
// and the date/time of release.
gst::plugin_define!(
    rtppace,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    env!("CARGO_PKG_VERSION"),
    "MPL-2.0",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    "2024-05-28"
);

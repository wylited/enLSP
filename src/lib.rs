pub fn version() -> String {
    use rustc_tools_util::VersionInfo;

    rustc_tools_util::get_version_info!().to_string()
}
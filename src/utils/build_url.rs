pub fn build_url(path: String, cli_token: String) -> String {
    let base_url = "https://central.dagpi.xyz";
    let self_path = "/self/";
    format!("{}{}{}/{}", base_url, self_path, path, cli_token)
}

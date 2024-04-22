use extism_pdk::*;
use fluentci_pdk::dag;

#[plugin_fn]
pub fn setup(version: String) -> FnResult<String> {
    let version = if version.is_empty() {
        "latest".into()
    } else {
        format!("{}", version)
    };

    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "install", &format!("dhall@{}", version)])?
        .stdout()?;

    Ok(stdout)
}

#[plugin_fn]
pub fn lint(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "dhall", "lint", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn freeze(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "dhall", "freeze", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn format(args: String) -> FnResult<String> {
    let stdout = dag()
        .pkgx()?
        .with_exec(vec!["pkgx", "dhall", "format", &args])?
        .stdout()?;
    Ok(stdout)
}

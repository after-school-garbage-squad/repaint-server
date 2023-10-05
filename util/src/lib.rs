use std::str::FromStr;

pub fn envvar<T, E>(var_name: &str, default: impl Into<Option<T>>) -> T
where
    T: std::fmt::Debug + FromStr<Err = E>,
    E: std::fmt::Debug,
{
    if let Ok(v) = std::env::var(var_name) {
        if v.is_empty() {
            panic!("{var_name} is set, but it's empty");
        }

        return v
            .parse()
            .unwrap_or_else(|e| panic!("environment variable {var_name} is not valid: {e:?}"));
    }
    if !cfg!(debug_assertions) {
        panic!("{var_name} is not set; Default environment variables are not allowed in the release build");
    }
    let Some(d) = default.into() else {
        panic!("{var_name} is not set");
    };
    tracing::warn!("environment variable {var_name} is not set. defaulting to {d:?}");

    d
}

pub fn envvar_str<'a>(var_name: &str, default: impl Into<Option<&'a str>>) -> String {
    if let Ok(v) = std::env::var(var_name) {
        if v.is_empty() {
            panic!("{var_name} is set, but it's empty");
        }

        return v;
    }
    if !cfg!(debug_assertions) {
        panic!("{var_name} is not set; Default environment variables are not allowed in the release build");
    }
    let Some(d) = default.into() else {
        panic!("{var_name} is not set");
    };
    tracing::warn!("environment variable {var_name} is not set. defaulting to {d:?}");

    d.to_owned()
}

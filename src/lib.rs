#[macro_use]
extern crate log;
#[macro_use]
extern crate simple_error; // for bail!

extern crate yaml_rust;
use std::error::Error;
use std::io::prelude::*;
use yaml_rust::YamlLoader;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn convert(yaml_str: String, filter: &[String]) -> Result<String> {
    let out = match YamlLoader::load_from_str(yaml_str.as_str()) {
        Ok(yaml) => yaml,
        Err(e) => bail!("unable to parse input as yaml: {}", e),
    };

    let first_doc = match out.into_iter().next() {
        Some(s) => s,
        None => bail!("input file contains no yaml documents"),
    };
    let hash = match first_doc.into_hash() {
        Some(s) => s,
        None => bail!("first yaml document is empty"),
    };

    let mut buf = Vec::new();
    buf.write_all(b"#!/usr/bin/env sh\n\n")?;

    for (k, v) in hash.into_iter() {
        if let Some(key) = k.into_string() {
            if filter.is_empty() || filter.contains(&key) {
                if let Some(value) = v.into_string() {
                    debug!("Written key '{}'", key);
                    // Using `{value:?}` would surround the contents with double-quotes
                    buf.write_fmt(format_args!("{key}=$(cat << '_EOF'\n{value}\n_EOF\n)\n\n"))?;
                }
            } else {
                debug!("Skipping key '{}'", key);
            }
        }
    }
    let result = String::from_utf8(buf)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use core::panic;

    // t_convert wraps `convert` to return error as string
    fn t_convert(yaml_str: &str) -> Result<String, String> {
        match super::convert(yaml_str.to_string(), Vec::<String>::new().as_slice()) {
            Ok(k) => Ok(k),
            Err(e) => Err(e.to_string()),
        }
    }

    #[test]
    fn convert_test1() {
        let expected = format!(
            "#!/usr/bin/env sh\n\n{name}=$(cat << '_EOF'\n{value}\n_EOF\n)\n\n",
            name = "VAR",
            value = "value"
        );
        if let Ok(value) = t_convert("VAR: value") {
            assert_eq!(value, expected);
        } else {
            panic!("Convert returned error")
        }
    }

    #[test]
    fn convert_test2() -> Result<(), String> {
        let result = t_convert("VAR: value")?;

        assert!(result.contains("#!/usr/bin/env sh"));
        assert!(result.contains("VAR"));
        assert!(result.contains("VAR="));
        assert!(result.contains("value"));
        Ok(())
    }

    #[test]
    fn convert_test3() -> Result<(), String> {
        let got = t_convert("VAR: value")?;
        let expected1 = format!(
            "{name}=$(cat << '_EOF'\n{value}\n_EOF\n)\n\n",
            name = "VAR",
            value = "value"
        );
        let expected2 = indoc::indoc! {r#"
        VAR=$(cat << '_EOF'
        value
        _EOF
        )

        "#};

        assert_eq!(expected1, expected2);

        assert!(got.contains(&expected1));
        assert!(got.contains(&expected2));
        Ok(())
    }
}

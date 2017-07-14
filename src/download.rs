use error::{self, ShadertoyError, InvalidShaderIdError, SaveShaderError};

use hyper::{self, Client};
use hyper::header::{Referer, ContentType};

use url::form_urlencoded;

use serde_json::{self, Value};

use std::io::{Read, Write};
use std::fs::File;

pub fn download(id: &str) -> error::Result<(String, String)> {
    let (name, code) = get_shader_name_and_code(id)?;

    let mut file = match File::create(&name) {
        Ok(file) => file,
        Err(err) => return Err(ShadertoyError::SaveShader(SaveShaderError::new(format!("Unable to save shader {}: {}", name, err)))),
    };

    if let Err(err) = file.write_all(&code.as_bytes()) {
        return Err(ShadertoyError::SaveShader(SaveShaderError::new(format!("Unable to save shader {}: {}", name, err))));
    }

    Ok((name, code))
}

fn get_shader_name_and_code(mut id: &str) -> error::Result<(String, String)> {
    let https_url = "https://www.shadertoy.com/view/";
    let http_url  = "http://www.shadertoy.com/view/";
    let url       = "www.shadertoy.com/view/";

    if id.starts_with(https_url) || id.starts_with(http_url) || id.starts_with(url) {
        id = id.split_at(id.rfind("view/").unwrap() + 5).1;
    }

    let json = serde_json::from_str::<Value>(&get_json_string(id)?)?;

    extract_from_json(&json)
}

fn get_json_string(id: &str) -> error::Result<String> {
    let client = Client::new();

    let body = form_urlencoded::Serializer::new(String::new())
        .extend_pairs(vec![("s", format!("{{\"shaders\": [\"{}\"]}}", id))])
        .finish();

    let mut res = client.post("https://www.shadertoy.com/shadertoy/")
        .header(Referer("https://www.shadertoy.com/".to_string()))
        .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
        .body(&body)
        .send()?;

    let mut buf = String::new();

    match res.read_to_string(&mut buf) {
        Ok(_) => {
            if buf == "[]" {
                let err = InvalidShaderIdError::new(format!("Shader '{}' not found", id));
                return Err(ShadertoyError::InvalidShaderId(err));
            } else {
                Ok(buf)
            }
        },
        Err(err) => {
            Err(ShadertoyError::DownloadShader(hyper::error::Error::from(err)))
        }
    }
}

fn extract_from_json(json: &Value) -> error::Result<(String, String)> {
    let name = format!("{}.frag", json[0]["info"]["name"].as_str().unwrap().replace(" ", "_")).to_lowercase();
    let mut code = String::new();

    let shaders = json[0]["renderpass"].as_array().unwrap();

    if shaders.len() > 1 {
        for shader in shaders {
            if shader["name"] == "Image" {
                code = String::from(shader["code"].as_str().unwrap());
            }
        }
    } else {
        code = String::from(shaders[0]["code"].as_str().unwrap());
    }

    Ok((name, code))
}
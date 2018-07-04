extern crate hyper;
extern crate hyper_tls;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate serde_urlencoded;
extern crate futures;

use futures::Future;
use futures::Stream;

pub struct Translate {
    api_key: String,
    client: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>
}

impl Translate {
    pub fn new(api_key: &str) -> Translate {
        Translate {
            api_key: api_key.to_owned(),
            client: hyper::Client::builder()
                    .build(hyper_tls::HttpsConnector::new(4).expect("Failed to initialize HttpsConnector"))
        }
    }

    pub fn translate(&self, text: &str, from_lang: &str, to_lang: &str) -> Box<Future<Item=String,Error=Error> + Send> {
        let uri = "https://translate.yandex.net/api/v1.5/tr.json/translate";

        let body = TranslateRequestBody {
            text: text.to_owned(),
            key: self.api_key.to_owned(),
            lang: format!("{}-{}", from_lang, to_lang)
        };

        let body = match serde_urlencoded::to_string(&body) {
            Ok(x) => x,
            Err(err) => return Box::new(futures::future::err(Error::Other(
                        format!("Failed to serialize request body: {:?}", err))))
        };

        let req = match hyper::Request::post(uri)
            .header(hyper::header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(hyper::header::CONTENT_LENGTH, body.len() as u64)
            .body(body.into()) {
                Ok(x) => x,
                Err(err) => return Box::new(futures::future::err(Error::Other(format!("Failed to construct request: {:?}", err))))
            };

        Box::new(self.client.request(req)
                 .map_err(|e| Error::Other(format!("Failed to send request: {:?}", e)))
                 .and_then(|res| {
                     res.into_body().concat2().map_err(|e| Error::Other(format!("Failed to get response: {:?}", e)))
                 })
                 .and_then(|res| {
                     let result = serde_json::from_slice(&res);
                     match result {
                         Ok(TranslateResponse::Error(err_res)) => Err(match err_res.code {
                             _ => Error::Other(format!("Unable to translate: {}", err_res.message))
                         }),
                         Ok(TranslateResponse::Success(resp)) => resp.text.into_iter().next().ok_or_else(|| Error::Other("No strings returned.".to_owned())),
                         Err(e) => Err(Error::Other(format!("Unable to parse response: {:?}", e)))
                     }
                 }))
    }
}

pub enum Error {
    Other(String)
}

impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        std::fmt::Display::fmt(self, fmt)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "{}", std::error::Error::description(self))
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref x) => &x
        }
    }
}

#[derive(Serialize)]
struct TranslateRequestBody {
    text: String,
    key: String,
    lang: String
}

#[derive(Deserialize)]
struct TranslateErrorResponse {
    code: u16,
    message: String
}

#[derive(Deserialize)]
struct TranslateSuccessResponse {
    text: Vec<String>
}

#[derive(Deserialize)]
#[serde(untagged)]
enum TranslateResponse {
    Error(TranslateErrorResponse),
    Success(TranslateSuccessResponse)
}

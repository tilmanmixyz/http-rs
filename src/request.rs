use crate::{header::HeaderMap, url::Url, Method};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Request {
    components: RequestComponents,
    body: String,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::default()
    }

    pub fn get(url: impl Into<String>) -> RequestBuilder {
        Request::builder().method(Method::Get).url(url)
    }

    pub fn post(url: impl Into<String>) -> RequestBuilder {
        Request::builder().method(Method::Post).url(url)
    }

    pub fn put(url: impl Into<String>) -> RequestBuilder {
        Request::builder().method(Method::Put).url(url)
    }

    pub fn delete(url: impl Into<String>) -> RequestBuilder {
        Request::builder().method(Method::Delete).url(url)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RequestBuilder {
    components: Option<RequestComponents>,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn components(&self) -> Option<RequestComponents> {
        self.components.clone()
    }

    pub fn method(mut self, method: Method) -> Self {
        let components = match self.components() {
            None => RequestComponents {
                method,
                ..Default::default()
            },
            Some(component) => RequestComponents {
                method,
                ..component
            },
        };

        self.components = Some(components);

        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        let url = url.into();
        let url = Url::new(url.as_str());
        let components = match self.components() {
            None => RequestComponents {
                url,
                ..Default::default()
            },
            Some(component) => RequestComponents { url, ..component },
        };

        self.components = Some(components);

        self
    }

    pub fn header(mut self, header: impl Into<String>, value: impl Into<String>) -> Self {
        let value = value.into();
        let h = header.into();
        let mut header_map = HeaderMap::new();
        header_map.insert(h, value);

        let header = header_map.into();

        let components = match self.components() {
            None => RequestComponents {
                header,
                ..Default::default()
            },
            Some(components) => {
                let header: HeaderMap = header
                    .get_map()
                    .to_owned()
                    .into_iter()
                    .chain(components.header.get_map().to_owned())
                    .collect();
                RequestComponents {
                    header: header.into(),
                    ..components
                }
            }
        };

        self.components = Some(components);
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RequestComponents {
    method: Method,
    url: crate::url::Url,
    header: crate::header::Header,
}

impl Default for RequestComponents {
    fn default() -> Self {
        Self {
            method: Method::Get,
            url: Url::new(""),
            header: Default::default(),
        }
    }
}

#[cfg(test)]
mod test_builder {
    use crate::url::Protocol;

    use super::*;

    #[test]
    fn url() {
        let builder = Request::builder().url("https://example.com/index/index.html");
        let url = builder.components().unwrap().url;
        assert_eq!(
            url.to_string(),
            String::from("https://example.com/index/index.html")
        );
        assert_eq!(url.protocol(), Protocol::Https);
        assert_eq!(url.host(), String::from("example.com"));
        assert_eq!(url.path(), String::from("/index/index.html"));
    }

    #[test]
    fn method() {
        let builder = Request::builder().method(Method::Delete);
        assert_eq!(builder.components().unwrap().method, Method::Delete);
    }

    #[test]
    fn header() {
        let builder = Request::builder()
            .header("Accept", "application/json")
            .header("Content-Type", "test/plain");
        let components = builder.components().unwrap();
        let header_map = components.header.get_map();
        assert!(&header_map.get("Content-Type").is_some());
        assert!(&header_map.get("Accept").is_some());
    }

    #[test]
    fn full() {
        let builder = Request::get("https://example.com/index/index.html")
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");
        let components = builder.components().unwrap();
        let header_map = components.header.get_map();
        assert!(&header_map.get_key_value("Accept").is_some());
        assert!(&header_map.get_key_value("Content-Type").is_some());
        assert_eq!(components.url.path(), String::from("/index/index.html"));
        assert_eq!(components.url.protocol(), Protocol::Https);
        assert_eq!(components.url.host(), String::from("example.com"));
        assert_eq!(components.method, Method::Get);
    }
}

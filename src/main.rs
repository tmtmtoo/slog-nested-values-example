use erased_serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate slog;
use slog_json;

use slog::Drain; // For slog::Fuse.

#[derive(Clone, Serialize)]
struct Wrapper<T>(T);

#[derive(Clone, Serialize)]
struct ObjectType {
    x: i64,
    y: String,
    z: Foo,
}

#[derive(Clone)]
struct Foo {
    v: Vec<u64>,
}

impl serde::Serialize for Foo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Foo", 1)?;
        state.serialize_field("v", "masking is possible!!!!")?;
        state.end()
    }
}

impl<T> slog::SerdeValue for Wrapper<T>
where
    T: serde::Serialize + Clone + Send + 'static,
{
    fn as_serde(&self) -> &dyn erased_serde::Serialize {
        self
    }

    fn to_sendable(&self) -> Box<dyn slog::SerdeValue + Send + 'static> {
        Box::new(self.clone())
    }
}

impl<T> slog::Value for Wrapper<T>
where
    T: serde::Serialize + Clone + Send + 'static,
{
    fn serialize(
        &self,
        _: &slog::Record<'_>,
        key: slog::Key,
        serializer: &mut dyn slog::Serializer,
    ) -> slog::Result {
        serializer.emit_serde(key, self)
    }
}

fn main() {
    // JSON Structured Log (STDOUT)
    let json_drain = slog_json::Json::default(std::io::stdout());

    // Support RUST_LOG Environment Variable
    let env_drain = slog_envlogger::new(json_drain);

    let drain = std::sync::Mutex::new(env_drain).map(slog::Fuse);

    let logger = slog::Logger::root(drain, slog_o!("version" => env!("CARGO_PKG_VERSION")));

    slog_info!(logger, "JSON list"; "list" => Wrapper(vec![123, 456]));
    slog_error!(logger, "JSON object"; "object" => Wrapper(ObjectType { x: 1, y: "test".to_string(), z: Foo {v: vec![0, 1, 2]} }));
}

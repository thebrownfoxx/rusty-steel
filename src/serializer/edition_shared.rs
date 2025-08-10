use domain::edition::edition_shared::EditionShared;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum SerializableEditionShared<T> {
    Same(T),
    Different { for_bedrock: T, for_java: T },
}

impl<T> SerializableEditionShared<T> {
    pub fn map<R, F>(self, f: F) -> SerializableEditionShared<R>
    where
        F: Fn(T) -> R,
    {
        match self {
            SerializableEditionShared::Same(value) => SerializableEditionShared::Same(f(value)),
            SerializableEditionShared::Different {
                for_bedrock,
                for_java,
            } => SerializableEditionShared::Different {
                for_bedrock: f(for_bedrock),
                for_java: f(for_java),
            },
        }
    }

    pub fn map_into<R: From<T>>(self) -> SerializableEditionShared<R> {
        self.map(|t| t.into())
    }
}

impl<T> From<SerializableEditionShared<T>> for EditionShared<T> {
    fn from(value: SerializableEditionShared<T>) -> Self {
        match value {
            SerializableEditionShared::Same(value) => EditionShared::Same(value),
            SerializableEditionShared::Different {
                for_bedrock,
                for_java,
            } => EditionShared::Different {
                for_bedrock,
                for_java,
            },
        }
    }
}

impl<T> From<EditionShared<T>> for SerializableEditionShared<T> {
    fn from(value: EditionShared<T>) -> Self {
        match value {
            EditionShared::Same(value) => SerializableEditionShared::Same(value),
            EditionShared::Different {
                for_bedrock,
                for_java,
            } => SerializableEditionShared::Different {
                for_bedrock,
                for_java,
            },
        }
    }
}

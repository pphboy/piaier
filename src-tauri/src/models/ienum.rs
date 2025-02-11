use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum SessionType {
    LONG,
    TEMP,
}

impl std::fmt::Display for SessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SessionType::LONG => write!(f, "LONG"),
            SessionType::TEMP => write!(f, "TEMP"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum MessageType {
    SYSTEM,
    USER,
    ASSISTANT,
}

impl std::fmt::Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::SYSTEM => write!(f, "SYSTEM"),
            MessageType::USER => write!(f, "USER"),
            MessageType::ASSISTANT => write!(f, "ASSISTANT"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum PrompterType {
    Chat,
    MultiModels,
    PyScript,
}

impl std::fmt::Display for PrompterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrompterType::Chat => write!(f, "CHAT"),
            PrompterType::MultiModels => write!(f, "MULTI_MODELS"),
            PrompterType::PyScript => write!(f, "PY_SCRIPT"),
        }
    }
}

pub enum MultiNodeType {
    NORMAL,
    SPEC_VAR,
}

impl std::fmt::Display for MultiNodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultiNodeType::NORMAL => write!(f, "NORMAL"),
            MultiNodeType::SPEC_VAR => write!(f, "SPEC_VAR"),
        }
    }
}

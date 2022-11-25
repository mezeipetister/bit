use crate::index::Index;

pub trait Cmd {
    const CMD_KEYWORD: &'static str;
    type ParamType;
    fn apply(db: &mut Index, params: Self::ParamType) -> Result<(), String>;
    fn get_keyword(&self) -> &'static str {
        Self::CMD_KEYWORD
    }
}

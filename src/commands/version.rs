use crate::cmd::Cmd;

pub struct Version;

impl Cmd for Version {
    const CMD_KEYWORD: &'static str = "version";

    type ParamType = ();

    fn apply(db: &mut crate::index::Index, params: Self::ParamType) -> Result<(), String> {
        todo!()
    }
}

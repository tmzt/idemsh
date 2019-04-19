
#[derive(Debug, PartialEq, Clone)]
pub enum IdemPathLocalPartType {
    Directory(String),
    File(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdemPath(pub Option<String>, pub IdemPathLocalPartType);

#[derive(Debug, PartialEq, Clone)]
pub enum IdemResourceType {
    Directory(String),
    Host(String),
    File(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdemValueType {
    LitString(String),
    ExtendedString(String),
    PathSpec(IdemPath),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdemReplace {
    regexp: String,
    replacement: String,
    global: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdemEditCommandType {
    InsertStart(String),
    InsertEnd(String),
    InsertAfter(String, String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdemEdit {
    commands: Vec<IdemEditCommandType>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdemParamType {
    FlagKeyword(String),
    ShortFlags(Vec<char>),
    KeyValue(String, IdemValueType),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IdemRawCommandWithPaths {
    pub paths: Vec<IdemPath>,
    pub params: Vec<IdemParamType>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IdemRawCommandType {
    Each(String, IdemValueType, Vec<Box<IdemRawCommandType>>),
    WithPaths(IdemRawCommandWithPaths),
    WithBlock(IdemResourceType, Option<String>, Vec<Box<IdemRawCommandType>>),
}

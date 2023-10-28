use std::num::ParseIntError;

#[derive(Debug)]
pub enum SolutionError {
    ReadFileErr(std::io::Error),
    GetLineErr(std::io::Error),
    ParseLineErr(ParseIntError),
    CharNotFoundErr(char),
    CouldNotFindPriorityErr,
    CouldNotSplitByCommaErr,
    CouldNotSplitByDashErr,
    CouldNotParseSectionStartValueErr(ParseIntError),
    CouldNotParseSectionEndValueErr(ParseIntError),
}

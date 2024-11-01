use nom::{
    IResult,
    bytes::complete::tag,
    sequence::preceded,
    character::complete::space1,
    combinator::rest,
};

pub fn parse_run_command(input: &str) -> IResult<&str, &str> {
    preceded(
        tag("run"),
        preceded(space1, rest)
    )(input)
}

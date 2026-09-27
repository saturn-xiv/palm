use askama::Template;

use super::super::task::Output;

#[derive(Template)]
#[template(path = "task-report/subject.txt", escape = "none")]
pub struct Subject<'a> {
    pub output: &'a Output,
    pub name: &'a str,
    pub version: &'a str,
}

#[derive(Template)]
#[template(path = "task-report/body.txt", escape = "none")]
pub struct Body<'a> {
    pub output: &'a Output,
    pub name: &'a str,
    pub version: &'a str,
    pub description: &'a str,
}

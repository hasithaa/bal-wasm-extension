pub mod ast;

use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{alpha1, alphanumeric1, multispace0, char, space0},
    sequence::{tuple, delimited, preceded},
    branch::alt,
    multi::{many0, separated_list0},
    combinator::{opt, map},
};

use crate::parser::ast::*;

pub struct Parser {
    source: String,
}

impl Parser {
    pub fn new(source: String) -> Self {
        Parser { source }
    }

    pub fn parse(&self) -> Result<ModuleNode, String> {
        match self.parse_module(self.source.as_str()) {
            Ok((_, module)) => Ok(module),
            Err(e) => Err(format!("Parse error: {:?}", e)),
        }
    }

    fn parse_module<'a>(&self, input: &'a str) -> IResult<&'a str, ModuleNode> {
        let (input, imports) = many0(|input| self.parse_import(input))(input)?;
        let (input, declarations) = many0(|input| self.parse_declaration(input))(input)?;
        
        Ok((input, ModuleNode {
            imports,
            declarations,
        }))
    }

    fn parse_declaration<'a>(&self, input: &'a str) -> IResult<&'a str, AstNode> {
        alt((
            map(|input| self.parse_function(input), AstNode::Function),
            map(|input| self.parse_service(input), AstNode::Service),
            map(|input| self.parse_variable(input), AstNode::Variable),
        ))(input)
    }

    fn parse_import<'a>(&self, input: &'a str) -> IResult<&'a str, ImportNode> {
        let (input, _) = tuple((
            tag("import"),
            multispace0,
        ))(input)?;
        
        let (input, path) = self.parse_qualified_identifier(input)?;
        let (input, alias) = opt(|input| self.parse_import_alias(input))(input)?;
        let (input, _) = char(';')(input)?;
        
        Ok((input, ImportNode {
            path,
            alias,
        }))
    }

    fn parse_qualified_identifier<'a>(&self, input: &'a str) -> IResult<&'a str, String> {
        separated_list0(
            char('/'),
            |input| self.parse_identifier(input)
        )(input)
        .map(|(input, parts)| (input, parts.join("/")))
    }

    fn parse_import_alias<'a>(&self, input: &'a str) -> IResult<&'a str, String> {
        let (input, _) = tuple((
            multispace0,
            tag("as"),
            multispace0,
        ))(input)?;
        
        self.parse_identifier(input)
    }

    fn parse_function<'a>(&self, input: &'a str) -> IResult<&'a str, FunctionNode> {
        let (input, visibility) = opt(preceded(
            space0,
            tag("public")
        ))(input)?;
        
        let (input, _) = tuple((
            multispace0,
            tag("function"),
            multispace0,
        ))(input)?;
        
        let (input, name) = self.parse_identifier(input)?;
        let (input, params) = self.parse_parameters(input)?;
        let (input, return_type) = opt(|input| self.parse_return_type(input))(input)?;
        let (input, body) = self.parse_function_body(input)?;

        Ok((input, FunctionNode {
            name,
            params,
            return_type,
            body,
            visibility: if visibility.is_some() { Visibility::Public } else { Visibility::Private },
        }))
    }

    fn parse_parameters<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<ParameterNode>> {
        delimited(
            char('('),
            separated_list0(
                tuple((char(','), multispace0)),
                |input| self.parse_parameter(input)
            ),
            char(')')
        )(input)
    }

    fn parse_parameter<'a>(&self, input: &'a str) -> IResult<&'a str, ParameterNode> {
        let (input, name) = self.parse_identifier(input)?;
        let (input, _) = tuple((multispace0, char(':'), multispace0))(input)?;
        let (input, param_type) = self.parse_type(input)?;
        
        Ok((input, ParameterNode {
            name,
            param_type,
        }))
    }

    fn parse_return_type<'a>(&self, input: &'a str) -> IResult<&'a str, TypeNode> {
        let (input, _) = tuple((
            multispace0,
            tag("returns"),
            multispace0,
        ))(input)?;
        
        self.parse_type(input)
    }

    fn parse_function_body<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<AstNode>> {
        delimited(
            char('{'),
            many0(|input| self.parse_statement(input)),
            char('}')
        )(input)
    }

    fn parse_statement<'a>(&self, input: &'a str) -> IResult<&'a str, AstNode> {
        // Simplified for this example
        map(
            |input| self.parse_variable(input),
            AstNode::Variable
        )(input)
    }

    fn parse_service<'a>(&self, input: &'a str) -> IResult<&'a str, ServiceNode> {
        let (input, _) = tuple((
            tag("service"),
            multispace0,
        ))(input)?;
        
        let (input, name) = self.parse_identifier(input)?;
        let (input, endpoints) = self.parse_service_endpoints(input)?;
        let (input, resources) = self.parse_service_body(input)?;
        
        Ok((input, ServiceNode {
            name,
            endpoints,
            resources,
        }))
    }

    fn parse_service_endpoints<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<String>> {
        preceded(
            tuple((multispace0, tag("on"), multispace0)),
            separated_list0(
                tuple((char(','), multispace0)),
                |input| self.parse_identifier(input)
            )
        )(input)
    }

    fn parse_service_body<'a>(&self, input: &'a str) -> IResult<&'a str, Vec<FunctionNode>> {
        delimited(
            char('{'),
            many0(|input| self.parse_function(input)),
            char('}')
        )(input)
    }

    fn parse_variable<'a>(&self, input: &'a str) -> IResult<&'a str, VariableNode> {
        let (input, name) = self.parse_identifier(input)?;
        let (input, _) = tuple((multispace0, char(':'), multispace0))(input)?;
        let (input, var_type) = self.parse_type(input)?;
        let (input, initializer) = opt(|input| self.parse_initializer(input))(input)?;
        
        Ok((input, VariableNode {
            name,
            var_type,
            initializer: initializer.map(Box::new),
        }))
    }

    fn parse_type<'a>(&self, input: &'a str) -> IResult<&'a str, TypeNode> {
        let (input, type_name) = self.parse_identifier(input)?;
        let (input, is_array) = opt(tag("[]"))(input)?;
        let (input, is_optional) = opt(char('?'))(input)?;
        
        Ok((input, TypeNode {
            type_name,
            is_array: is_array.is_some(),
            is_optional: is_optional.is_some(),
        }))
    }

    fn parse_initializer<'a>(&self, input: &'a str) -> IResult<&'a str, AstNode> {
        let (input, _) = tuple((multispace0, char('='), multispace0))(input)?;
        // Simplified - just parse an identifier as a variable reference
        map(
            |input| self.parse_identifier(input),
            |name| AstNode::Variable(VariableNode {
                name,
                var_type: TypeNode {
                    type_name: "unknown".to_string(),
                    is_array: false,
                    is_optional: false,
                },
                initializer: None,
            })
        )(input)
    }

    fn parse_identifier<'a>(&self, input: &'a str) -> IResult<&'a str, String> {
        let (input, first) = alpha1(input)?;
        let (input, rest) = many0(alt((alphanumeric1, tag("_"))))(input)?;
        
        let mut ident = first.to_string();
        for r in rest {
            ident.push_str(r);
        }
        
        Ok((input, ident))
    }
}

use serde::Deserialize;

use garnish_lang::compiler::lex::lex;
use garnish_lang::compiler::parse::parse;
use garnish_lang::compiler::build::build_with_data;
use garnish_lang::simple::{SimpleGarnishRuntime, SimpleGarnishData, SimpleRuntimeState};
use garnish_lang::{EmptyContext, GarnishData, GarnishRuntime};
use serde_garnish::GarnishDataDeserializer;

use crate::css::RuleSet;
use crate::html::*;

pub fn make_html_from_garnish(input: &str) -> Result<Node, String> {
    let tokens = lex(input)?;
    let parsed = parse(&tokens)?;
    let mut data = SimpleGarnishData::new();
    build_with_data(parsed.get_root(), parsed.get_nodes().clone(), &mut data)?;
    let mut runtime = SimpleGarnishRuntime::new(data);
    runtime.get_data_mut().push_value_stack(0)?;

    loop {
        match runtime.execute_current_instruction::<EmptyContext>(None) {
            Err(e) => Err(e)?,
            Ok(data) => match data.get_state() {
                SimpleRuntimeState::Running => (),
                SimpleRuntimeState::End => break,
            },
        }
    }

    let mut deserializer = GarnishDataDeserializer::new(runtime.get_data_mut());

    let result = Node::deserialize(&mut deserializer).map_err(|e| e.to_string())?;

    return Ok(result);
}

pub fn make_css_from_garnish(input: &str) -> Result<RuleSet, String> {
    let tokens = lex(input)?;
    let parsed = parse(&tokens)?;
    let mut data = SimpleGarnishData::new();
    build_with_data(parsed.get_root(), parsed.get_nodes().clone(), &mut data)?;
    let mut runtime = SimpleGarnishRuntime::new(data);
    runtime.get_data_mut().push_value_stack(0)?;

    loop {
        match runtime.execute_current_instruction::<EmptyContext>(None) {
            Err(e) => Err(e)?,
            Ok(data) => match data.get_state() {
                SimpleRuntimeState::Running => (),
                SimpleRuntimeState::End => break,
            },
        }
    }

    let mut deserializer = GarnishDataDeserializer::new(runtime.get_data_mut());

    let result = RuleSet::deserialize(&mut deserializer).map_err(|e| match e.message() {
        Some(m) => m.clone(),
        None => e.to_string(),
    })?;

    return Ok(result);
}

#[cfg(test)]
mod test {
    use crate::css::{
        Declaration, DeclarationValue, MediaCondition, MediaConstraint, MediaFeature, MediaQuery,
        Rule, RuleSet, Selector,
    };
    use crate::html::Node;
    use crate::{make_css_from_garnish, make_html_from_garnish};

    #[test]
    fn make_node() {
        let input = ";Node::Text, \"This is a text node\"";
        let output = make_html_from_garnish(input).unwrap();

        assert_eq!(output, Node::Text("This is a text node".to_string()))
    }

    #[test]
    fn make_rule_set() {
        let input = "
;rules = (
    (
        ;selector = (;Selector::Tag \"body\"),
        ;declarations = (
            (
                ;property = \"color\",
                ;value = (;DeclarationValue::Basic \"blue\")
            ),
        )
    ),
),";
        let output = make_css_from_garnish(input).unwrap();

        assert_eq!(
            output,
            RuleSet::new(
                vec![Rule::new(
                    Selector::Tag("body".to_string()),
                    vec![Declaration::new(
                        "color".to_string(),
                        DeclarationValue::Basic("blue".to_string())
                    )],
                    vec![]
                )],
                vec![],
                None
            )
        )
    }

    #[test]
    fn make_rule_set_all_fields() {
        let input = "
;media_query = (
    ;media_type = \"screen\",
    ;constraint = ;MediaConstraint::Only,
    ;features = (
        (
            ;MediaCondition::And
            (
                (;property = \"max-width\" ;value = \"1000px\"),
                (;property = \"orientation\" ;value = \"landscape\")
            )
        ),
    ),
),
;rules = (
    (
        ;selector = (;Selector::Tag \"body\"),
        ;declarations = (
            (
                ;property = \"color\",
                ;value = (;DeclarationValue::Basic \"blue\")
            ),
        ),
        ;sub_rules = (
            (
                ;selector = (;Selector::Tag \"h1\"),
                ;declarations = (
                    (
                        ;property = \"color\",
                        ;value = (;DeclarationValue::Basic \"red\")
                    ),
                ),
            ),
        )
    ),
),
;sub_sets = (
    (
        ;media_query = (
            ;media_type = \"print\",
            ;constraint = ;MediaConstraint::Not,
            ;features = (
                (
                    ;MediaCondition::Or
                    (
                        (;property = \"max-width\" ;value = \"1000px\"),
                        (;property = \"orientation\" ;value = \"landscape\")
                    )
                ),
            )
        ),
        ;rules = (
            (
                ;selector = (;Selector::Tag \"body\"),
                ;declarations = (
                    (
                        ;property = \"color\",
                        ;value = (;DeclarationValue::Basic \"green\")
                    ),
                )
            ),
        ),
    ),
)
";
        let output = make_css_from_garnish(input).unwrap();

        assert_eq!(
            output,
            RuleSet::new(
                vec![Rule::new(
                    Selector::Tag("body".to_string()),
                    vec![Declaration::new(
                        "color".to_string(),
                        DeclarationValue::Basic("blue".to_string())
                    )],
                    vec![Rule::new(
                        Selector::Tag("h1".to_string()),
                        vec![Declaration::new(
                            "color".to_string(),
                            DeclarationValue::Basic("red".to_string())
                        )],
                        vec![]
                    )]
                )],
                vec![RuleSet::new(
                    vec![Rule::new(
                        Selector::Tag("body".to_string()),
                        vec![Declaration::new(
                            "color".to_string(),
                            DeclarationValue::Basic("green".to_string())
                        )],
                        vec![]
                    )],
                    vec![],
                    Some(MediaQuery::new(
                        MediaConstraint::Not,
                        "print".to_string(),
                        vec![MediaCondition::Or(
                            MediaFeature::new("max-width".to_string(), "1000px".to_string()),
                            MediaFeature::new("orientation".to_string(), "landscape".to_string())
                        )]
                    ))
                )],
                Some(MediaQuery::new(
                    MediaConstraint::Only,
                    "screen".to_string(),
                    vec![MediaCondition::And(
                        MediaFeature::new("max-width".to_string(), "1000px".to_string()),
                        MediaFeature::new("orientation".to_string(), "landscape".to_string())
                    )]
                ))
            )
        )
    }
}

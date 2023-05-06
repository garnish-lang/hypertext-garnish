use serde::Deserialize;

use garnish_data::SimpleRuntimeData;
use garnish_lang_compiler::{build_with_data, lex, parse};
use garnish_lang_runtime::runtime_impls::SimpleGarnishRuntime;
use garnish_traits::{EmptyContext, GarnishLangRuntimeState, GarnishRuntime};
use serde_garnish::GarnishDataDeserializer;
use crate::css::RuleSet;

use crate::html::*;

pub fn make_html_from_garnish(input: &str) -> Result<Node, String> {
    let tokens = lex(input)?;
    let parsed = parse(tokens)?;
    let mut data = SimpleRuntimeData::new();
    build_with_data(parsed.get_root(), parsed.get_nodes().clone(), &mut data)?;
    let mut runtime = SimpleGarnishRuntime::new(data);
    loop {
        match runtime.execute_current_instruction::<EmptyContext>(None) {
            Err(e) => Err(e)?,
            Ok(data) => match data.get_state() {
                GarnishLangRuntimeState::Running => (),
                GarnishLangRuntimeState::End => break,
            },
        }
    }

    let mut deserializer = GarnishDataDeserializer::new(runtime.get_data_mut());

    let result = Node::deserialize(&mut deserializer).map_err(|e| e.to_string())?;

    return Ok(result);
}

pub fn make_css_from_garnish(input: &str) -> Result<RuleSet, String> {
    let tokens = lex(input)?;
    let parsed = parse(tokens)?;
    let mut data = SimpleRuntimeData::new();
    build_with_data(parsed.get_root(), parsed.get_nodes().clone(), &mut data)?;
    let mut runtime = SimpleGarnishRuntime::new(data);
    loop {
        match runtime.execute_current_instruction::<EmptyContext>(None) {
            Err(e) => Err(e)?,
            Ok(data) => match data.get_state() {
                GarnishLangRuntimeState::Running => (),
                GarnishLangRuntimeState::End => break,
            },
        }
    }

    let mut deserializer = GarnishDataDeserializer::new(runtime.get_data_mut());

    let result = RuleSet::deserialize(&mut deserializer).map_err(|e| match e.message() {
        Some(m) => m.clone(),
        None => e.to_string()
    })?;

    return Ok(result);
}

#[cfg(test)]
mod test {
    use crate::css::{Declaration, DeclarationValue, Rule, RuleSet, Selector};
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
            (
                ;property = \"background-color\",
                ;value = (;DeclarationValue::Basic \"red\")
            )
        )
    ),
    (
        ;selector = (;Selector::Tag \"body\"),
        ;declarations = (
            (
                ;property = \"color\",
                ;value = (;DeclarationValue::Basic \"blue\")
            ),
            (
                ;property = \"background-color\",
                ;value = (;DeclarationValue::Basic \"red\")
            )
        )
    ),
),
;sub_sets = (,)
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
                    vec![]
                )],
                vec![],
                None
            )
        )
    }
}

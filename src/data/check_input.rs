use crate::data::model::DataObject;

#[derive(PartialEq, Debug)]
pub enum InputCase {
    DupQuotes,
    InputInvalid,
    PathExists,
    Normal,
}

pub fn check_input(d: &DataObject, p: String, k: String) -> InputCase {
    let case: InputCase;
    if let Some(existings) = d.targets.get(&k) {
        if existings == &p || d.targets.contains_key(&k) {
            case = InputCase::PathExists;
        } else {
            case = InputCase::Normal;
        }
    } else {
        if [k.clone(), p.clone()]
            .iter()
            .any(|s| s.contains("\'") || s.contains("\"") || s.contains("\\") || s.contains("/"))
        {
            if [k.clone(), p.clone()].iter().any(|s| {
                s.starts_with("\"") && s.ends_with("\"") || s.starts_with("\'") && s.ends_with("\'")
            }) {
                case = InputCase::DupQuotes;
            } else {
                case = InputCase::Normal;
            }
        } else if p.contains("\"") || p.contains("\'") {
            case = InputCase::InputInvalid;
        } else {
            case = InputCase::Normal;
        }
    }
    println!("{:?}", case);
    case
}

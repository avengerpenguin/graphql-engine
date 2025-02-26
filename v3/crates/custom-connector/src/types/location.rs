use std::collections::BTreeMap;

use ndc_models;

pub(crate) fn definition() -> ndc_models::ObjectType {
    ndc_models::ObjectType {
        description: Some("A location".into()),
        fields: BTreeMap::from_iter([
            (
                "city".into(),
                ndc_models::ObjectField {
                    description: Some("The location's city".into()),
                    r#type: ndc_models::Type::Named {
                        name: "String".into(),
                    },
                    arguments: BTreeMap::new(),
                },
            ),
            (
                "country".into(),
                ndc_models::ObjectField {
                    description: Some("The location's country".into()),
                    r#type: ndc_models::Type::Named {
                        name: "String".into(),
                    },
                    arguments: BTreeMap::new(),
                },
            ),
            (
                "campuses".into(),
                ndc_models::ObjectField {
                    description: Some("The location's campuses".into()),
                    r#type: ndc_models::Type::Array {
                        element_type: Box::new(ndc_models::Type::Named {
                            name: "String".into(),
                        }),
                    },
                    arguments: BTreeMap::new(),
                },
            ),
        ]),
    }
}

use std::collections::BTreeMap;

use ndc_models;

pub(crate) fn definition() -> ndc_models::ObjectType {
    ndc_models::ObjectType {
        description: Some("A movie genre".into()),
        fields: BTreeMap::from_iter([
            (
                "id".into(),
                ndc_models::ObjectField {
                    description: Some("The genre's primary key".into()),
                    r#type: ndc_models::Type::Named { name: "Int".into() },
                    arguments: BTreeMap::new(),
                },
            ),
            (
                "name".into(),
                ndc_models::ObjectField {
                    description: Some("The genre's name".into()),
                    r#type: ndc_models::Type::Named {
                        name: "String".into(),
                    },
                    arguments: BTreeMap::new(),
                },
            ),
            (
                "movies".into(),
                ndc_models::ObjectField {
                    description: Some("Notable movies of this genre".into()),
                    r#type: ndc_models::Type::Array {
                        element_type: Box::new(ndc_models::Type::Named {
                            name: "movie".into(),
                        }),
                    },
                    arguments: BTreeMap::new(),
                },
            ),
        ]),
    }
}

use shopify_function::prelude::*;
use shopify_function::Result;

generate_types!(
    query_path = "src/run.graphql",
    schema_path = "schema.graphql"
);

#[shopify_function]
fn function(input: input::ResponseData) -> Result<output::FunctionResult> {
    let has_bulky_variant = input.cart.lines.into_iter().any(|line| {
        match &line.merchandise {
            input::InputCartLinesMerchandise::ProductVariant(variant) => variant.product.has_any_tag,
            input::InputCartLinesMerchandise::CustomProduct => false,
        }
    });

    let cost;
    let pickup_instruction;
    if has_bulky_variant {
        cost = Decimal(2.99);
        pickup_instruction = String::from("Ready for pickup next business day.");
    } else {
        cost = Decimal(0.00);
        pickup_instruction = String::from("Ready for pickup now!");
    }

    let operations = input.locations
        .into_iter()
        .map(|location| {
            output::Operation {
                add: output::LocalPickupDeliveryOption {
                    title: Some(location.name.clone()),
                    cost: Some(cost),
                    pickup_location: output::PickupLocation {
                        location_handle: location.handle.clone(),
                        pickup_instruction: Some(pickup_instruction.clone()),
                    },
                    metafields: Some(vec![])
                }
            }
        })
        .collect();

    return Ok(output::FunctionResult { operations })
}

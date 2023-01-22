use qrcode::Color;
use serde::{Deserialize, Serialize};

use crate::{domain::commands, infra::uuid::Uuid};

#[derive(Debug, Deserialize, Serialize)]
pub struct StartPixPaymentInput {
    pub creator_id: Uuid,
    pub subscription: SubscriptionInput,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SubscriptionInput {
    Monthly,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct StartPixPaymentOutput {
    pub qrcode: Vec<u8>,
}

impl From<StartPixPaymentInput> for commands::pix_payment::StartPixPaymentInput {
    fn from(input: StartPixPaymentInput) -> Self {
        Self {
            creator_id: input.creator_id,
            subscription: match input.subscription {
                SubscriptionInput::Monthly => commands::pix_payment::SubscriptionInput::Monthly,
            },
        }
    }
}

impl From<commands::pix_payment::StartPixPaymentOutput> for StartPixPaymentOutput {
    fn from(input: commands::pix_payment::StartPixPaymentOutput) -> Self {
        Self {
            qrcode: input
                .qrcode
                .into_iter()
                .map(|color| match color {
                    Color::Dark => 0,
                    Color::Light => 1,
                })
                .collect(),
        }
    }
}

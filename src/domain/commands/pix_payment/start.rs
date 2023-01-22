use crate::{
    domain::contracts::{context::Context, deps::Deps},
    infra::uuid::Uuid,
};
use anyhow::{Context as anyhowContext, Result};
use qrcode::{Color, QrCode};

#[derive(Debug)]
pub struct StartPixPaymentInput {
    pub creator_id: Uuid,
    pub subscription: SubscriptionInput,
}

#[derive(Debug)]
pub enum SubscriptionInput {
    Monthly,
}

#[derive(Debug)]
pub struct StartPixPaymentOutput {
    pub qrcode: Vec<Color>,
}

#[tracing::instrument(name = "commands::pix_payment::start_payment", skip_all, fields(
    ctx = ?ctx,
    input = ?input
))]
pub async fn start_payment(
    _deps: &Deps,
    ctx: &Context,
    input: StartPixPaymentInput,
) -> Result<StartPixPaymentOutput> {
    let code =
        QrCode::new(b"https://www.youtube.com/watch?v=zAmHQk-OW3k").context("generating qrcode")?;

    code.to_colors();

    Ok(StartPixPaymentOutput {
        qrcode: code.to_colors(),
    })
}

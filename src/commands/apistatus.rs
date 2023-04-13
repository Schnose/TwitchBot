use {
	crate::{error::Result, state::Context},
	gokz_rs::global_api::{self, HealthReport},
};

pub async fn apistatus(ctx: Context<'_>) -> Result<()> {
	let HealthReport { successful_responses, fast_responses } =
		global_api::checkhealth(&ctx.gokz_client).await?;

	let avg = (successful_responses as f64 + fast_responses as f64) / 2f64;
	let success = (avg * 10.0) as u8;

	let emote = match success {
		90.. => "FeelsGoodMan",
		67.. => "Susge",
		33.. => "monkaS",
		_ => "Deadge",
	};

	let message = format!(
		"{} {}/{} Successful Responses | {}/{} Fast Responses",
		emote, successful_responses, 10, fast_responses, 10
	);

	ctx.reply(message).await
}

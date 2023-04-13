use {
	crate::{error::Result, state::Context},
	gokz_rs::global_api,
	schnosebot::time,
};

pub async fn record(ctx: Context<'_>, record_id: u32) -> Result<()> {
	let record = global_api::get_record(record_id, &ctx.gokz_client).await?;

	let player = record.player_name;
	let map = record.map_name;
	let mode = record.mode.short();
	let runtype = if record.teleports > 0 { "TP" } else { "PRO" };
	let time = time::format(record.time);
	let teleports = match record.teleports {
		0 => String::new(),
		1 => String::from(" (1 TP)"),
		n => format!(" ({n} TPs)"),
	};
	let date = record
		.created_on
		.format("%d/%m/%Y %H:%M:%S");

	ctx.reply(format!("[{player} on {map} ({mode} {runtype})] {time}{teleports} | {date}"))
		.await
}

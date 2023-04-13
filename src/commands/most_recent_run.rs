use {
	crate::{error::Result, state::Context},
	gokz_rs::schnose_api,
	schnosebot::time,
};

pub async fn most_recent_run(ctx: Context<'_>) -> Result<()> {
	let most_recent_run = schnose_api::get_records(1, &ctx.gokz_client)
		.await?
		.remove(0);

	let player_name = most_recent_run.player.name;
	let map = most_recent_run.map_name;
	let mode = most_recent_run.mode.short();
	let runtype = if most_recent_run.teleports > 0 { "TP" } else { "PRO" };
	let time = time::format(most_recent_run.time);
	let teleports = match most_recent_run.teleports {
		0 => String::new(),
		1 => String::from(" (1 TP)"),
		n => format!(" ({n} TPs)"),
	};
	let date = most_recent_run
		.created_on
		.format("%d/%m/%Y %H:%M:%S");

	ctx.reply(format!("[{player_name} on {map} ({mode} {runtype})] {time}{teleports} | {date}",))
		.await
}

use {
	crate::{error::Result, state::Context},
	gokz_rs::{schnose_api, PlayerIdentifier},
	schnosebot::time,
};

pub async fn recent(ctx: Context<'_>, player: PlayerIdentifier) -> Result<()> {
	let recent = schnose_api::get_recent(player, 1, &ctx.gokz_client)
		.await?
		.remove(0);

	let player = recent.player.name;
	let map = recent.map_name;
	let mode = recent.mode.short();
	let runtype = if recent.teleports > 0 { "TP" } else { "PRO" };
	let time = time::format(recent.time);
	let teleports = match recent.teleports {
		0 => String::new(),
		1 => String::from(" (1 TP)"),
		n => format!(" ({n} TPs)"),
	};
	let date = recent
		.created_on
		.format("%d/%m/%Y %H:%M:%S");

	ctx.reply(format!("[{player} on {map} ({mode} {runtype})] {time}{teleports} | {date}"))
		.await
}

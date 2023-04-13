use {
	crate::{error::Result, state::Context},
	gokz_rs::{
		global_api, {MapIdentifier, Mode},
	},
	schnosebot::{global_map::GlobalMap, time},
};

pub async fn wr(ctx: Context<'_>, map: GlobalMap, mode: Mode) -> Result<()> {
	let map = MapIdentifier::Name(map.name);

	let tp_wr = global_api::get_wr(map.clone(), mode, true, 0, &ctx.gokz_client)
		.await
		.ok();

	let pro_wr = global_api::get_wr(map.clone(), mode, false, 0, &ctx.gokz_client)
		.await
		.ok();

	if tp_wr.is_none() && pro_wr.is_none() {
		return ctx.reply("No WRs found.").await;
	}

	let tp_time = tp_wr
		.map(|wr| {
			let player_name = wr.player_name;
			let time = time::format(wr.time);
			let teleports = match wr.teleports {
				1 => String::from("(1 TP)"),
				n => format!("({n} TPs)"),
			};
			format!("{time} {teleports} by {player_name}")
		})
		.unwrap_or_else(|| String::from("no record"));

	let pro_time = pro_wr
		.map(|wr| {
			let player_name = wr.player_name;
			let time = time::format(wr.time);
			format!("{time} by {player_name}")
		})
		.unwrap_or_else(|| String::from("no record"));

	ctx.reply(format!(
		"[WR on {map} ({mode})] TP: {tp_time} | PRO: {pro_time}",
		mode = mode.short()
	))
	.await
}

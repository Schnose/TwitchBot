use {
	crate::{error::Result, state::Context},
	gokz_rs::{
		global_api, {MapIdentifier, Mode, PlayerIdentifier},
	},
	schnosebot::{global_map::GlobalMap, time},
};

pub async fn bpb(
	ctx: Context<'_>,
	map: GlobalMap,
	course: u8,
	mode: Mode,
	player: PlayerIdentifier,
) -> Result<()> {
	let map = MapIdentifier::Name(map.name);

	let tp_pb =
		global_api::get_pb(player.clone(), map.clone(), mode, true, course, &ctx.gokz_client)
			.await
			.ok();

	let pro_pb =
		global_api::get_pb(player.clone(), map.clone(), mode, false, course, &ctx.gokz_client)
			.await
			.ok();

	if tp_pb.is_none() && pro_pb.is_none() {
		return ctx.reply("No BPBs found.").await;
	}

	let mut player_name = player.to_string();

	let tp_time = tp_pb
		.map(|pb| {
			player_name = pb.player_name;
			let time = time::format(pb.time);
			let teleports = match pb.teleports {
				1 => String::from("(1 TP)"),
				n => format!("({n} TPs)"),
			};
			format!("{time} {teleports}")
		})
		.unwrap_or_else(|| String::from("no record"));

	let pro_time = pro_pb
		.map(|pb| {
			player_name = pb.player_name;
			time::format(pb.time)
		})
		.unwrap_or_else(|| String::from("no record"));

	ctx.reply(format!(
		"[{player_name} on {map} B{course} ({mode})] TP: {tp_time} | PRO: {pro_time}",
		mode = mode.short()
	))
	.await
}

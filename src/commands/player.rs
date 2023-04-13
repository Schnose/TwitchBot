use {
	crate::{error::Result, state::Context},
	gokz_rs::{
		schnose_api::{self, FancyPlayer},
		PlayerIdentifier,
	},
};

pub async fn player(ctx: Context<'_>, player: PlayerIdentifier) -> Result<()> {
	let FancyPlayer { name, steam_id, records, .. } =
		schnose_api::get_player(player, &ctx.gokz_client).await?;

	let total = records.total;
	let kzt_tp = records.kzt.tp;
	let kzt_pro = records.kzt.pro;
	let skz_tp = records.skz.tp;
	let skz_pro = records.skz.pro;
	let vnl_tp = records.vnl.tp;
	let vnl_pro = records.vnl.pro;

	ctx.reply(format!(
		"[{name} ({steam_id})] {total} Total Records \
		 | KZT: {kzt_tp} TP / {kzt_pro} PRO \
		 | SKZ: {skz_tp} TP / {skz_pro} PRO \
		 | VNL: {vnl_tp} TP / {vnl_pro} PRO"
	))
	.await
}

use {
	crate::{error::Result, state::Context},
	schnosebot::global_map::GlobalMap,
};

pub async fn map(
	ctx: Context<'_>,
	GlobalMap {
		name,
		tier,
		courses,
		mapper_name,
		updated_on,
		..
	}: GlobalMap,
) -> Result<()> {
	let tier = tier as u8;
	let bonuses = match courses.len() {
		2 => String::from("1 Bonus"),
		n => {
			let n = n - 1;
			format!("{n} Bonuses")
		}
	};
	let mapper = mapper_name;
	let last_update = updated_on.format("%d/%m/%Y");

	ctx.reply(format!(
		"{name} (T{tier}) - {bonuses} - Made by {mapper} - Last Updated {last_update}",
	))
	.await
}

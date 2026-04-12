use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use serde_with::{BoolFromInt, serde_as};
use std::time::Duration;

fn no_map_name() -> String {
	"N/A".to_owned()
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct ServerStatus {
	pub version: String,
	#[serde_as(as = "BoolFromInt")]
	pub respawn: bool,
	pub round_id: String,
	pub players: usize,
	pub revision: String,
	pub revision_date: String,
	pub admins: usize,
	pub gamestate: GameState,
	#[serde(default = "no_map_name")]
	pub map_name: String,
	pub security_level: String,
	#[serde(deserialize_with = "deserialize_time_safe")]
	pub round_duration: Duration,
	#[serde(default, flatten)]
	pub time_dilation: TimeDilationStats,
	#[serde(flatten)]
	pub shuttle_info: Option<ShuttleInfo>,
}

#[derive(Debug, Default, Deserialize, Clone, Copy)]
pub struct TimeDilationStats {
	#[serde(rename = "time_dilation_current")]
	pub current: f32,
	#[serde(rename = "time_dilation_avg")]
	pub average: f32,
	#[serde(rename = "time_dilation_avg_slow")]
	pub average_slow: f32,
	#[serde(rename = "time_dilation_avg_fast")]
	pub average_fast: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Deserialize_repr)]
#[repr(u8)]
pub enum GameState {
	Startup = 0,
	Pregame = 1,
	SettingUp = 2,
	Playing = 3,
	Finished = 4,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ShuttleInfo {
	pub shuttle_mode: ShuttleMode,
	#[serde(deserialize_with = "deserialize_time_safe")]
	pub shuttle_timer: Duration,
	#[serde(rename = "shuttle_emergency_reason")]
	pub reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ShuttleMode {
	Idle,
	Igniting,
	#[serde(rename = "recalled")]
	Recall,
	#[serde(rename = "called")]
	Call,
	Docked,
	Stranded,
	Disabled,
	#[serde(rename = "escape")]
	Escape,
	#[serde(rename = "endgame: game over")]
	Endgame,
	Recharging,
	#[serde(rename = "landing")]
	PreArrival,
}

fn deserialize_time_safe<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
	D: Deserializer<'de>,
{
	let seconds = i64::deserialize(deserializer)?;
	Ok(Duration::from_secs(seconds.max(0) as u64))
}

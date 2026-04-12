use crate::colors::Color;
use serde::{Deserialize, Deserializer};
use serde_repr::Deserialize_repr;
use std::time::Duration;

fn no_map_name() -> String {
	"N/A".to_owned()
}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerStatus {
	pub version: String,
	#[serde(deserialize_with = "deserialize_bool")]
	pub respawn: bool,
	pub round_id: String,
	pub players: usize,
	pub revision: String,
	pub revision_date: String,
	pub admins: usize,
	#[serde(rename = "gamestate")]
	pub game_state: GameState,
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

impl ServerStatus {
	/// Checks to see if the emergency shuttle is currently on its way or not.
	pub fn is_shuttle_coming(&self) -> bool {
		matches!(
			self.shuttle_info,
			Some(ShuttleInfo {
				shuttle_mode: ShuttleMode::Call
					| ShuttleMode::Docked
					| ShuttleMode::Igniting
					| ShuttleMode::Escape
					| ShuttleMode::PreArrival,
				..
			})
		)
	}

	pub fn has_round_started(&self) -> bool {
		matches!(self.game_state, GameState::Playing | GameState::Finished)
	}

	pub fn tidi_color(&self) -> Color {
		match self.time_dilation.current {
			..5.0 => Color::Green,
			5.0..10.0 => Color::Yellow,
			_ => Color::Red,
		}
	}

	pub fn game_state_color(&self) -> Color {
		match self.game_state {
			GameState::Startup | GameState::SettingUp | GameState::Pregame => Color::Green,
			GameState::Playing => Color::Default,
			GameState::Finished => Color::Red,
		}
	}

	pub fn security_color(&self) -> Color {
		match self.security_level.as_str() {
			"green" => Color::Green,
			"blue" => Color::Blue,
			"red" => Color::Red,
			"yellow" | "amber" => Color::Yellow,
			"delta" | "gamma" | "epsilon" | "lambda" => Color::Magenta,
			_ => Color::Default,
		}
	}
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

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
	D: Deserializer<'de>,
{
	f32::deserialize(deserializer).map(|f| f != 0.0)
}

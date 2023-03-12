use std::{collections::HashMap, cmp};

use reqwest::{header, Response, Error};
use serde::{Serialize, Deserialize, de};


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum BalanceState {
    #[default]
    OffPlatform,
    OnPlatform,
    OnDocked,
    NotAttempt,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
enum MatchType {
    #[default]
    Friendly,
    Quarter,
    Semi,
    Final,
}

// scouter	eventCode	matchLevel	matchNumber	match_key	robot	teamNumber	autoStartingLocation	autoScoredGrid	autoCrossedCable	autoCrossedChargingStation	autoMobility	autoDocked	cycleTimes	scoredGrid	feedCount	wasFed	wasDefended	whoDefended	smartLinks	floorPickUp	dockingTime	finalState	numOfRobotsDocked	driverSkill	linksScored	defenseRating	swerveDrive	speedRating	diedOrTipped	tippy	droppedCones	goodPartner	comments	autoGamePieces	autoCubes	autoCones	autoHigh	autoMed	autoLow	avgCycleTime	teleopGamePieces	teleopCubes	teleopCones	teleopHigh	teleopMed	teleopLow	totalGamePieces	totalCubes	totalCones	totalHigh	totalMed	totalLow


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MatchEntry {
    pub scouter: String,
    pub event_code: String,
    #[serde(deserialize_with = "from_match_type_string")]
    match_level: MatchType,
    pub match_number: u64,
    #[serde(rename = "match_key")]
    pub match_key: String,
    pub robot: String,
    pub team_number: u64,
    pub auto_starting_location: String,
    pub auto_scored_grid: String,
    #[serde(deserialize_with = "from_bool_string")]
    pub auto_crossed_cable: bool,
    #[serde(deserialize_with = "from_bool_string")]
    pub auto_crossed_charging_station: bool,
    #[serde(deserialize_with = "from_bool_string")]
    pub auto_mobility: bool,
    #[serde(deserialize_with = "from_charge_station_int")]
    auto_docked: BalanceState,
    pub cycle_times: String,
    pub scored_grid: String,
    pub feed_count: u64,
    #[serde(deserialize_with = "from_bool_string")]
    pub was_fed: bool,
    #[serde(deserialize_with = "from_bool_string")]
    pub was_defended: bool,
    pub who_defended: String,
    pub smart_links: u64,
    pub floor_pick_up: String,
    pub docking_time: f64,
    #[serde(deserialize_with = "from_charge_station_int")]
    final_state: BalanceState,
    pub num_of_robots_docked: u64,
    pub driver_skill: String,
    pub links_scored: u64,
    pub defense_rating: String,
    #[serde(deserialize_with = "from_bool_string")]
    pub swerve_drive: bool,
    pub speed_rating: u64,
    #[serde(deserialize_with = "from_bool_string")]
    pub died_or_tipped: bool,
    #[serde(deserialize_with = "from_bool_string")]
    pub tippy: bool,
    #[serde(deserialize_with = "from_bool_string")]
    pub dropped_cones: bool,
    #[serde(deserialize_with = "from_bool_string")]
    pub good_partner: bool,
    pub comments: String,
    pub auto_game_pieces: u64,
    pub auto_cubes: u64,
    pub auto_cones: u64,
    pub auto_high: u64,
    pub auto_med: u64,
    pub auto_low: u64,
    pub avg_cycle_time: String,
    pub teleop_game_pieces: u64,
    pub teleop_cubes: u64,
    pub teleop_cones: u64,
    pub teleop_high: u64,
    pub teleop_med: u64,
    pub teleop_low: u64,
    pub total_game_pieces: u64,
    pub total_cubes: u64,
    pub total_cones: u64,
    pub total_high: u64,
    pub total_med: u64,
    pub total_low: u64,
}

impl MatchEntry {
    pub fn constrain_values(&mut self) -> MatchEntry {

        self.to_owned()
    }
}

fn empty_tba_data() -> Option<String> {
    None
}

fn from_bool_string<'de, D>(
    deserializer: D,
) -> Result<bool, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let s: &str =
        de::Deserialize::deserialize(deserializer).unwrap_or("false");
    
    match s {
        "true"|"TRUE"|"1" => Ok(true),
        "false"|"FALSE"|"0" => Ok(false),
        _ => Ok(false),
         //_ => Err(de::Error::custom(s.to_owned()+" is not a valid boolean"))
        
    }
}

fn from_charge_station_int<'de, D>(
    deserializer: D,
) -> Result<BalanceState, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let num: &str = 
        de::Deserialize::deserialize(deserializer)?;
    match num {
        "0" | "x" | "a" | "OffPlatform" => Ok(BalanceState::OffPlatform),
        "1" | "d" | "OnPlatform" => Ok(BalanceState::OnPlatform),
        "2" | "e" | "OnDocked" => Ok(BalanceState::OnDocked),
        _ => Ok(BalanceState::NotAttempt),
        //_ => Err(de::Error::custom("Not a valid Balance Status"))
    }
}

fn from_match_type_string<'de, D>(
    deserializer: D,
) -> Result<MatchType, D::Error>
where 
    D: de::Deserializer<'de>,
{
    let string: &str =
        de::Deserialize::deserialize(deserializer)?;
    
    match string {
        _ => Ok(MatchType::Friendly),
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TeamSummary {
    pub team_number: u64,
    pub avg_cone_low: f64,
    pub avg_cone_med: f64,
    pub avg_cone_high: f64,
    pub avg_cube_low: f64,
    pub avg_cube_med: f64,
    pub avg_cube_high: f64,
    pub avg_low: f64,
    pub avg_med: f64,
    pub avg_high: f64,
    pub avg_links: f64,
    pub can_balance: bool,
    pub balance_percentage: f64,
    pub dock_percentage: f64,
    pub auto_mobility: bool,
}

// UNSURE OF IMPLEMENTATION FOR AVERAGING
struct  TeamSummaryAvgCounter {
    avg_low: Vec<u64>,
    avg_med: Vec<u64>,
    avg_high: Vec<u64>,
    avg_links: Vec<u64>,
    balance_count: Vec<u64>,
    dock_count: Vec<u64>

}
impl TeamSummaryAvgCounter {
    pub fn new() -> TeamSummaryAvgCounter {
        TeamSummaryAvgCounter { avg_low: Vec::new(), avg_med: Vec::new(), avg_high: Vec::new(), balance_count: Vec::new(), dock_count: Vec::new(), avg_links: Vec::new() }
    }
}


impl TeamSummary {
    pub fn new(team: &FrcTeam) -> TeamSummary {
        let mut avg_count = TeamSummaryAvgCounter::new();
        let mut balance_flag = false;
        let mut mobility_flag = false;
        for match_entry in &team.match_data {
            avg_count.avg_low.push(match_entry.total_low);
            avg_count.avg_med.push(match_entry.total_med);
            avg_count.avg_high.push(match_entry.total_high);
            avg_count.avg_links.push(match_entry.links_scored);

            match match_entry.final_state {
                BalanceState::OffPlatform => {
                    avg_count.balance_count.push(0);
                    avg_count.dock_count.push(0);
                }
                
                BalanceState::OnDocked => {
                    avg_count.balance_count.push(0);
                    avg_count.dock_count.push(1);
                    balance_flag = true;
                }
                
                BalanceState::OnPlatform => {
                    avg_count.balance_count.push(1);
                    avg_count.dock_count.push(0);
                    balance_flag = true;
                }
                BalanceState::NotAttempt => {}
            }
            if (match_entry.auto_mobility) {mobility_flag = true};
            
        }

        

        

        TeamSummary { 
            team_number: team.team_number, 
            avg_cone_low: 0 as f64, 
            avg_cone_med: 0 as f64, 
            avg_cone_high: 0 as f64, 
            avg_cube_low: 0 as f64, 
            avg_cube_med: 0 as f64,  
            avg_cube_high: 0 as f64,
            avg_low: avg_count.avg_low.iter().copied().sum::<u64>() as f64 / avg_count.avg_low.len() as f64,
            can_balance: balance_flag,
            avg_med: avg_count.avg_med.iter().copied().sum::<u64>() as f64 /avg_count.avg_med.len() as f64,
            avg_high: avg_count.avg_high.iter().copied().sum::<u64>() as f64 /avg_count.avg_high.len() as f64,
            balance_percentage: avg_count.balance_count.iter().copied().sum::<u64>() as f64 / avg_count.balance_count.len() as f64,
            dock_percentage: avg_count.dock_count.iter().copied().sum::<u64>() as f64 / avg_count.dock_count.len() as f64,
            avg_links: avg_count.avg_links.iter().copied().sum::<u64>() as f64 / avg_count.avg_links.len() as f64,
            auto_mobility: mobility_flag,
        }

    }
    // Creates a combination of two teams into one summary
    fn combine_teams(team1: &TeamSummary, team2: &TeamSummary) -> TeamSummary {
        TeamSummary {
            team_number: team1.team_number,
            avg_cone_low: (team1.avg_cone_low + team2.avg_cone_low),
            avg_cone_med: (team1.avg_cone_med + team2.avg_cone_med),
            avg_cone_high: (team1.avg_cone_high + team2.avg_cone_high),
            avg_cube_low: (team1.avg_cube_low + team2.avg_cube_low),
            avg_cube_med: (team1.avg_cube_med + team2.avg_cube_med),
            avg_cube_high: (team1.avg_cube_high + team2.avg_cube_high),
            avg_low: (team1.avg_low + team2.avg_low),
            avg_med: (team1.avg_med + team2.avg_med),
            avg_high: (team1.avg_high + team2.avg_high),
            can_balance: team1.can_balance || team2.can_balance,
            balance_percentage: f64::max(team1.balance_percentage, team2.balance_percentage),
            dock_percentage: f64::max(team1.dock_percentage, team2.dock_percentage),
            avg_links: team1.avg_links + team2.avg_links,
            auto_mobility: team1.auto_mobility | team2.auto_mobility
        }
    }
    pub fn constrain_values(&mut self) -> Self {
        self.avg_cone_low = self.avg_cone_low.clamp(0.0, 9.0);
        self.avg_cone_med = self.avg_cone_med.clamp(0.0, 6.0);
        self.avg_cone_high = self.avg_cone_high.clamp(0.0, 6.0);
        self.avg_cube_low = self.avg_cube_low.clamp(0.0, 9.0);
        self.avg_cube_med = self.avg_cube_med.clamp(0.0, 3.0);
        self.avg_cube_high = self.avg_cube_high.clamp(0.0, 3.0);
        self.avg_low = (self.avg_low).clamp(0.0, 9.0);
        self.avg_med = (self.avg_med).clamp(0.0, 9.0);
        self.avg_high = (self.avg_high).clamp(0.0, 9.0);
        self.dock_percentage = self.dock_percentage.clamp(0.0, 1.0);
        self.balance_percentage = self.balance_percentage.clamp(0.0, 1.0);
        self.to_owned()
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct RankMaxCount {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub balance: f64,
    pub dock: f64
}

pub struct PointValues {
    pub low: f64,
    pub medium: f64,
    pub high: f64,
    pub balance: f64,
    pub dock: f64
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct RankOptions {
    pub comparison_team: Option<FrcTeam>
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct TeamRanking {
    pub team_number: u64,
    pub overall_rating: f64,
    pub low_rating: f64,
    pub medium_rating: f64,
    pub high_rating: f64,
    pub balance_rating: f64,
    pub dock_rating: f64,
    pub data_reliability_rating: f64
}



const MAX_SCORE_COLUMNS: f64 = 9.0;


impl TeamRanking {
    pub fn generate_rankings(teams: HashMap<u64, FrcTeam>, options: RankOptions) -> Vec<TeamRanking> {
        let mut maxCount = RankMaxCount::default();
        let mut rankings = Vec::new();
        let mut comparison_team: FrcTeam;
        if options.comparison_team.is_none() { // Comparison Team is the team that is being added to each team to get the rating as if two teams were together
            comparison_team = FrcTeam::default();
        } else {
            comparison_team = options.comparison_team.unwrap();
        }

        // TODO: Make these better to configure
        let point_values = PointValues {
            low: 2.0,
            medium: 3.0,
            high: 5.0,
            balance: 6.0, // This is the remainder of Dock so its not in the total_points
            dock: 10.0
        };

        // TODO: Optimize to not iterate through all teams twice
        for mut team in teams.values() {
            if (comparison_team.team_number == team.team_number) {
                continue;
            }
            if (comparison_team.get_summary().is_none()) {
                comparison_team.summary = Some(TeamSummary::default());
            }  // Stupid hack to make sure comparison team has a summary
            let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap()).constrain_values();
            if team_summary.avg_low > maxCount.low {
                maxCount.low = team_summary.avg_low;
            }
            if team_summary.avg_med > maxCount.medium {
                maxCount.medium = team_summary.avg_med;
            }
            if team_summary.avg_high > maxCount.high {
                maxCount.high = team_summary.avg_high;
            }
            if team_summary.balance_percentage > maxCount.balance {
                maxCount.balance = team_summary.balance_percentage;
            }
            if team_summary.dock_percentage > maxCount.dock {
                maxCount.dock = team_summary.dock_percentage;
            }
        };

        let totalPoints = (maxCount.low*point_values.low + maxCount.medium*point_values.medium + maxCount.high*point_values.high + maxCount.balance*point_values.balance + maxCount.dock*point_values.dock);

        for team in teams.values() {
            if (comparison_team.team_number == team.team_number) {
                continue;
            }
            let team_summary = TeamSummary::combine_teams(team.get_summary().as_ref().unwrap(), comparison_team.get_summary().as_ref().unwrap()).constrain_values();
            let mut ranking = TeamRanking::default();
            ranking.team_number = team.team_number;
            ranking.low_rating = team_summary.avg_low / maxCount.low;
            ranking.medium_rating = team_summary.avg_med / maxCount.medium;
            ranking.high_rating = team_summary.avg_high / maxCount.high;
            ranking.balance_rating = team_summary.balance_percentage / maxCount.balance;
            ranking.dock_rating = team_summary.dock_percentage / maxCount.dock;
            ranking.data_reliability_rating = 1.0;
            ranking.overall_rating = (team_summary.avg_low*point_values.low + team_summary.avg_med*point_values.medium + team_summary.avg_high*point_values.high + team_summary.balance_percentage*point_values.balance + team_summary.dock_percentage*point_values.dock)/totalPoints;
            rankings.push(ranking);
        };
        rankings

    }
        
}

#[derive(Debug, Default, Clone , Serialize, Deserialize)]
pub struct FrcTeam {
    version_id: u64,
    pub team_number: u64,
    match_data: Vec<MatchEntry>,
    pub summary: Option<TeamSummary>,
    tba_data: Option<HashMap<String, serde_json::Value>>
}

impl FrcTeam {
    pub fn new(team_number: u64) -> FrcTeam {
        FrcTeam { version_id: 1, team_number: team_number, match_data: Vec::new(), summary: None, tba_data: None} 
    }

    pub fn generate_summary(&mut self) {
        self.summary = Some(TeamSummary::new(&self).constrain_values());
    }

    pub fn get_summary(&self) -> &Option<TeamSummary> {
        &self.summary
    }

    pub fn get_mut_summary(&mut self) -> &Option<TeamSummary> {
        &mut self.summary
    }

    pub fn query_tba_data(&mut self, auth_key: &str) {
        self.tba_data = match get_tba_data(auth_key, &("/team/frc".to_owned()+&self.team_number.to_string())) {
            Ok(data) => Some(data.json::<HashMap<String, serde_json::Value>>().unwrap()),
            Err(err) => None
        };
    }

    pub fn add_match_entry(&mut self, mut entry: MatchEntry) {
        self.match_data.push(entry.constrain_values());
    }
}


fn get_tba_data(auth_key:&str, query:&str) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let request_url = "https://www.thebluealliance.com/api/v3".to_string()+query;
    let client = reqwest::blocking::Client::new();
    let response =  client.get(request_url)
        .header("X-TBA-Auth-Key", auth_key)
        .send();
    response    
}

#[cfg(test)]
mod tests {
}
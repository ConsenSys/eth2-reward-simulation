////////////////////////////////////////////////////////////////////////////////
//
// Output stores the outcomes from the simulation of an epoch
//
////////////////////////////////////////////////////////////////////////////////
use super::config::*;
use super::deltas::Deltas;

const MONTHS_PER_YEAR: i32 = 12;

pub struct Output {
    pub rows: Vec<EpochReportRow>,
}

impl Output {
    pub fn new() -> Output {
        let rows = vec![];

        Output { rows: rows }
    }

    pub fn push(&mut self, row: EpochReportRow) {
        self.rows.push(row);
    }

    pub fn print_epoch_report(&self, mode: &str) {
        if mode == "csv" {
            println!(
                "{},{},{},{},{},{},{},{}",
                "epoch number".to_string(),
                "FFG rewards".to_string(),
                "FFG penalties".to_string(),
                "proposer rewards".to_string(),
                "attester rewards".to_string(),
                "total staked balance".to_string(),
                "total effective balance".to_string(),
                "sim time (μs)".to_string(),
            );

            for row in &self.rows {
                println!(
                    "{},{},{},{},{},{},{},{}",
                    row.epoch_id,
                    row.deltas_head_ffg_rewards,
                    row.deltas_head_ffg_penalties,
                    row.deltas_proposer_rewards,
                    row.deltas_attester_rewards,
                    row.total_staked_balance,
                    row.total_effective_balance,
                    row.time_elapsed,
                );
            }
        }
    }

    pub fn print_monthly_report(&self, config: &Config) {
        let epochs_per_year = config.epochs;
        let epochs_per_month = epochs_per_year / MONTHS_PER_YEAR;

        let mut monthly_report: Vec<MonthlyReportRow> = Vec::new();
        let mut items_to_get = vec![];

        for epoch in (epochs_per_month..epochs_per_year).step_by(epochs_per_month as usize) {
            items_to_get.push(epoch)
        }

        for (index, item) in items_to_get.iter().enumerate() {
            let current_item = &self.rows[*item as usize];
            let _network_percentage_rewards = ((current_item.total_staked_balance as f64
                - config.total_at_stake_initial as f64)
                / config.total_at_stake_initial as f64)
                * 100f64;
            let _network_percentage_penalties = ((current_item.deltas_head_ffg_penalties as f64)
                / config.total_at_stake_initial as f64)
                * 100f64;
            let _network_percentage_net_rewards =
                _network_percentage_rewards - _network_percentage_penalties;

            monthly_report.push(MonthlyReportRow {
                month_number: index as u32 + 1u32,
                network_percentage_rewards: _network_percentage_rewards,
                network_percentage_penalties: _network_percentage_penalties,
                network_percentage_net_rewards: _network_percentage_net_rewards,
            });
        }

        for record in monthly_report {
            println!(
                "Month number: {}, Total Network Rewards {}",
                record.month_number, record.network_percentage_net_rewards
            );
        }
    }
}

#[derive(Copy, Clone)]
pub struct MonthlyReportRow {
    pub month_number: u32,
    pub network_percentage_rewards: f64,
    pub network_percentage_penalties: f64,
    pub network_percentage_net_rewards: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct EpochReportRow {
    pub epoch_id: i32,

    pub deltas_head_ffg_rewards: u64,
    pub deltas_head_ffg_penalties: u64,
    pub deltas_proposer_rewards: u64,
    pub deltas_attester_rewards: u64,

    pub total_staked_balance: u64,
    pub total_effective_balance: u64,

    pub time_elapsed: u128,
}

impl EpochReportRow {
    pub fn new() -> EpochReportRow {
        EpochReportRow {
            epoch_id: 0,

            deltas_head_ffg_rewards: 0,
            deltas_head_ffg_penalties: 0,
            deltas_proposer_rewards: 0,
            deltas_attester_rewards: 0,

            total_staked_balance: 0,
            total_effective_balance: 0,

            time_elapsed: 0,
        }
    }

    pub fn aggregate(&mut self, deltas: &Deltas) {
        self.deltas_head_ffg_rewards += deltas.head_ffg_reward;
        self.deltas_head_ffg_penalties += deltas.head_ffg_penalty;
        self.deltas_proposer_rewards += deltas.proposer_reward;
        self.deltas_attester_rewards += deltas.attester_reward;
    }
}

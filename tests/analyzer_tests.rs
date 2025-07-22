use crate::models::HistoryEntry;
use crate::analyzer::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_of_day_stats_empty() {
        let history: Vec<HistoryEntry> = vec![];
        time_of_day_stats(&history); // Should not panic
    }

    #[test]
    fn test_heatmap_stats_empty() {
        let history: Vec<HistoryEntry> = vec![];
        heatmap_stats(&history); // Should not panic
    }
}

// This file is intentionally left empty. Analyzer tests are now in src/analyzer.rs under #[cfg(test)].

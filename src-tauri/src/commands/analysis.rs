//! Analysis execution commands
//!
//! Orchestrates the IP correlation analysis workflow.

use bankflow_core::{IpMatcher, Processor};
use crate::core::whois::WhoisClient;
use crate::models::AnalysisResult;
use crate::state::AppState;
use tauri::{AppHandle, Emitter, State};

/// Progress event payload
#[derive(Clone, serde::Serialize)]
struct ProgressPayload {
    stage: String,
    progress: u32,
    message: String,
}

/// Run the IP correlation analysis
#[tauri::command]
pub async fn run_analysis(
    app: AppHandle,
    state: State<'_, AppState>,
    hide_sensitive: bool,
    split_income_expense: bool,
    _ip_cross_reference: bool, // Reserved for future use
    whois_lookup: bool,
) -> Result<AnalysisResult, String> {
    // Check if files are loaded
    let transactions = state.transactions.read().await;
    let ip_records = state.ip_records.read().await;

    if transactions.is_empty() {
        return Err("No transactions loaded. Please load File A first.".to_string());
    }

    if ip_records.is_empty() {
        return Err("No IP records loaded. Please load File B first.".to_string());
    }

    // Clone data for processing
    let mut tx_data = transactions.clone();
    let ip_data = ip_records.clone();

    // Release locks
    drop(transactions);
    drop(ip_records);

    let total_records = tx_data.len();

    // Stage 1: IP Matching
    emit_progress(&app, "matching", 0, "Starting IP matching...");

    let matcher = IpMatcher::with_default_window(&ip_data);
    matcher.match_all(&mut tx_data);

    let stats = matcher.get_stats(&tx_data);
    emit_progress(&app, "matching", 100, &format!("Matched {} records", stats.matched));

    // Stage 2: Sensitive Column Processing
    if hide_sensitive {
        emit_progress(&app, "processing", 0, "Hiding sensitive columns...");
        let processor = Processor::new(true);
        processor.process(&mut tx_data);
        emit_progress(&app, "processing", 100, "Sensitive columns hidden");
    }

    // Stage 3: Whois Lookup (if enabled)
    let mut whois_queried = 0;
    if whois_lookup {
        emit_progress(&app, "whois", 0, "Starting Whois lookup...");

        let mut whois_client = WhoisClient::new();
        whois_client.enrich_transactions(&mut tx_data).await;

        let (total_cached, successful) = whois_client.cache_stats();
        whois_queried = total_cached;

        emit_progress(
            &app,
            "whois",
            100,
            &format!("Whois complete: {} queries ({} successful)", total_cached, successful),
        );
    }

    // Stage 4: Split income/expense
    let (income, expense) = if split_income_expense {
        emit_progress(&app, "splitting", 0, "Splitting income/expense...");
        let result = Processor::split_income_expense(&tx_data);
        emit_progress(
            &app,
            "splitting",
            100,
            &format!("Split: {} income, {} expense", result.0.len(), result.1.len()),
        );
        result
    } else {
        (vec![], vec![])
    };

    // Store results in state
    {
        let mut results = state.results.write().await;
        results.summary = tx_data.clone();
        results.income = income.clone();
        results.expense = expense.clone();
        results.is_complete = true;
        results.total_records = total_records;
        results.matched_count = stats.matched;
        results.multi_ip_count = stats.multi_ip;
        results.whois_queried = whois_queried;
    }

    emit_progress(&app, "complete", 100, "Analysis complete!");

    Ok(AnalysisResult {
        total_records,
        matched_count: stats.matched,
        multi_ip_count: stats.multi_ip,
        whois_queried,
        settings: crate::models::AnalysisSettings {
            hide_sensitive,
            split_income_expense,
            ip_cross_reference: _ip_cross_reference,
            whois_lookup,
        },
    })
}

/// Emit progress event to frontend
fn emit_progress(app: &AppHandle, stage: &str, progress: u32, message: &str) {
    let _ = app.emit(
        "analysis-progress",
        ProgressPayload {
            stage: stage.to_string(),
            progress,
            message: message.to_string(),
        },
    );
}

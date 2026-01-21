//! Data preprocessing module

use crate::models::Transaction;

const SENSITIVE_COLUMNS: [usize; 4] = [2, 5, 11, 12];

pub struct Processor {
    hide_sensitive: bool,
    sensitive_columns: Vec<usize>,
}

impl Processor {
    pub fn new(hide_sensitive: bool) -> Self {
        Self {
            hide_sensitive,
            sensitive_columns: SENSITIVE_COLUMNS.to_vec(),
        }
    }

    pub fn process(&self, transactions: &mut [Transaction]) {
        if self.hide_sensitive {
            self.hide_columns(transactions);
        }
    }

    fn hide_columns(&self, transactions: &mut [Transaction]) {
        let mut indices = self.sensitive_columns.clone();
        indices.sort_by(|a, b| b.cmp(a));

        for tx in transactions.iter_mut() {
            for &idx in &indices {
                if idx < tx.raw_columns.len() {
                    tx.raw_columns.remove(idx);
                }
            }
        }
    }

    pub fn split_income_expense(
        transactions: &[Transaction],
    ) -> (Vec<Transaction>, Vec<Transaction>) {
        let income: Vec<Transaction> = transactions
            .iter()
            .filter(|tx| tx.income.map_or(false, |v| v > 0.0))
            .cloned()
            .collect();

        let expense: Vec<Transaction> = transactions
            .iter()
            .filter(|tx| tx.expense.map_or(false, |v| v > 0.0))
            .cloned()
            .collect();

        (income, expense)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProcessingStats {
    pub total: usize,
    pub income_count: usize,
    pub expense_count: usize,
    pub total_income: f64,
    pub total_expense: f64,
}

impl ProcessingStats {
    pub fn from_transactions(transactions: &[Transaction]) -> Self {
        let mut stats = ProcessingStats::default();
        stats.total = transactions.len();

        for tx in transactions {
            if let Some(income) = tx.income {
                if income > 0.0 {
                    stats.income_count += 1;
                    stats.total_income += income;
                }
            }
            if let Some(expense) = tx.expense {
                if expense > 0.0 {
                    stats.expense_count += 1;
                    stats.total_expense += expense;
                }
            }
        }

        stats
    }
}

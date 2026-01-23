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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_transaction(income: Option<f64>, expense: Option<f64>) -> Transaction {
        Transaction {
            datetime: None,
            timestamp: "2024-01-15 10:30:00".to_string(),
            account: "ACC001".to_string(),
            income,
            expense,
            matched_ip: None,
            ip_country: None,
            ip_isp: None,
            raw_columns: vec![
                "col0".to_string(),
                "col1".to_string(),
                "col2".to_string(), // sensitive
                "col3".to_string(),
                "col4".to_string(),
                "col5".to_string(), // sensitive
                "col6".to_string(),
            ],
            row_index: 1,
        }
    }

    #[test]
    fn test_processor_hide_sensitive() {
        let mut transactions = vec![create_test_transaction(None, None)];
        assert_eq!(transactions[0].raw_columns.len(), 7);

        let processor = Processor::new(true);
        processor.process(&mut transactions);

        // Should have removed columns 2 and 5 (indices within bounds)
        assert!(transactions[0].raw_columns.len() < 7);
    }

    #[test]
    fn test_processor_no_hide() {
        let mut transactions = vec![create_test_transaction(None, None)];
        let original_len = transactions[0].raw_columns.len();

        let processor = Processor::new(false);
        processor.process(&mut transactions);

        // Should not change anything
        assert_eq!(transactions[0].raw_columns.len(), original_len);
    }

    #[test]
    fn test_split_income_expense() {
        let transactions = vec![
            create_test_transaction(Some(1000.0), None),
            create_test_transaction(None, Some(500.0)),
            create_test_transaction(Some(200.0), Some(100.0)), // Both
            create_test_transaction(None, None),
        ];

        let (income, expense) = Processor::split_income_expense(&transactions);

        assert_eq!(income.len(), 2); // 2 transactions with income > 0
        assert_eq!(expense.len(), 2); // 2 transactions with expense > 0
    }

    #[test]
    fn test_processing_stats() {
        let transactions = vec![
            create_test_transaction(Some(1000.0), None),
            create_test_transaction(None, Some(500.0)),
            create_test_transaction(Some(200.0), Some(100.0)),
        ];

        let stats = ProcessingStats::from_transactions(&transactions);

        assert_eq!(stats.total, 3);
        assert_eq!(stats.income_count, 2);
        assert_eq!(stats.expense_count, 2);
        assert!((stats.total_income - 1200.0).abs() < 0.01);
        assert!((stats.total_expense - 600.0).abs() < 0.01);
    }
}

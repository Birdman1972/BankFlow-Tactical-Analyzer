//! Excel report exporter
//!
//! Outputs to bytes for WASM compatibility (can also save to file on native).

use crate::error::CoreError;
use crate::models::Transaction;
use rust_xlsxwriter::{Color, Format, FormatBorder, Workbook, Worksheet};

const HEADERS: &[&str] = &[
    "Row", "Timestamp", "Account", "Expense", "Income", "Matched IP", "Country", "ISP",
];

pub struct Exporter;

impl Exporter {
    /// Export to bytes (WASM compatible)
    pub fn export_to_bytes(
        summary: &[Transaction],
        income: &[Transaction],
        expense: &[Transaction],
    ) -> Result<Vec<u8>, CoreError> {
        let mut workbook = Workbook::new();

        let header_format = Format::new()
            .set_bold()
            .set_background_color(Color::RGB(0x1F2937))
            .set_font_color(Color::RGB(0x00FF9D))
            .set_border(FormatBorder::Thin);
        let data_format = Format::new().set_border(FormatBorder::Thin);
        let money_format = Format::new()
            .set_num_format("#,##0.00")
            .set_border(FormatBorder::Thin);
        let ip_format = Format::new()
            .set_font_color(Color::RGB(0x00D2FF))
            .set_border(FormatBorder::Thin);
        let multi_ip_format = Format::new()
            .set_font_color(Color::RGB(0xFF0055))
            .set_bold()
            .set_border(FormatBorder::Thin);

        // Summary sheet
        {
            let ws = workbook.add_worksheet();
            ws.set_name("Summary").map_err(|e| CoreError::ExportError(e.to_string()))?;
            write_headers(ws, &header_format)?;
            write_transactions(ws, summary, &data_format, &money_format, &ip_format, &multi_ip_format)?;
            set_column_widths(ws)?;
        }

        // Income sheet
        {
            let ws = workbook.add_worksheet();
            ws.set_name("Income").map_err(|e| CoreError::ExportError(e.to_string()))?;
            write_headers(ws, &header_format)?;
            write_transactions(ws, income, &data_format, &money_format, &ip_format, &multi_ip_format)?;
            set_column_widths(ws)?;
        }

        // Expense sheet
        {
            let ws = workbook.add_worksheet();
            ws.set_name("Expense").map_err(|e| CoreError::ExportError(e.to_string()))?;
            write_headers(ws, &header_format)?;
            write_transactions(ws, expense, &data_format, &money_format, &ip_format, &multi_ip_format)?;
            set_column_widths(ws)?;
        }

        let buffer = workbook
            .save_to_buffer()
            .map_err(|e| CoreError::ExportError(format!("Failed to save: {}", e)))?;

        Ok(buffer)
    }
}

fn write_headers(ws: &mut Worksheet, fmt: &Format) -> Result<(), CoreError> {
    for (col, header) in HEADERS.iter().enumerate() {
        ws.write_string_with_format(0, col as u16, *header, fmt)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;
    }
    Ok(())
}

fn write_transactions(
    ws: &mut Worksheet,
    transactions: &[Transaction],
    data_fmt: &Format,
    money_fmt: &Format,
    ip_fmt: &Format,
    multi_ip_fmt: &Format,
) -> Result<(), CoreError> {
    for (row_idx, tx) in transactions.iter().enumerate() {
        let row = row_idx as u32 + 1;

        ws.write_number_with_format(row, 0, tx.row_index as f64, data_fmt)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;
        ws.write_string_with_format(row, 1, &tx.timestamp, data_fmt)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;
        ws.write_string_with_format(row, 2, &tx.account, data_fmt)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;

        if let Some(expense) = tx.expense {
            ws.write_number_with_format(row, 3, expense, money_fmt)
                .map_err(|e| CoreError::ExportError(e.to_string()))?;
        }
        if let Some(income) = tx.income {
            ws.write_number_with_format(row, 4, income, money_fmt)
                .map_err(|e| CoreError::ExportError(e.to_string()))?;
        }

        let ip_str = tx.matched_ip.as_deref().unwrap_or("N/A");
        let fmt = if ip_str.contains(" | ") { multi_ip_fmt } else { ip_fmt };
        ws.write_string_with_format(row, 5, ip_str, fmt)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;

        ws.write_string_with_format(row, 6, tx.ip_country.as_deref().unwrap_or(""), data_fmt)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;
        ws.write_string_with_format(row, 7, tx.ip_isp.as_deref().unwrap_or(""), data_fmt)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;
    }
    Ok(())
}

fn set_column_widths(ws: &mut Worksheet) -> Result<(), CoreError> {
    let widths = [8, 20, 15, 12, 12, 40, 10, 20];
    for (col, width) in widths.iter().enumerate() {
        ws.set_column_width(col as u16, *width)
            .map_err(|e| CoreError::ExportError(e.to_string()))?;
    }
    Ok(())
}

"""
專案名稱：BankFlow Tactical Analyzer (Web 戰術版)
描述：協助執法單位進行離線數位鑑識分析，透過網頁介面清洗、整理並交叉比對銀行存款與網銀 IP 紀錄。
版本：1.0.0-web
作者：Antigravity AICoder
日期：2026-01-09
技術：Streamlit + Pandas
執行方式：streamlit run main.py
"""

import streamlit as st
import pandas as pd
import requests
import io
import gc
import datetime
import time

# -----------------------------------------------------------------------------
# 頁面配置與 CSS 樣式 (賽博龐克風格)
# -----------------------------------------------------------------------------
st.set_page_config(
    page_title="BankFlow Tactical Analyzer",
    page_icon="🛡️",
    layout="wide",
    initial_sidebar_state="expanded"
)

# 自定義 CSS
CYBERPUNK_CSS = """
<style>
    /* 全局背景色 */
    .stApp {
        background-color: #111827;
        color: #E5E7EB;
    }
    
    /* 標題樣式 */
    h1, h2, h3 {
        color: #00FF9D !important;
        font-family: 'Courier New', Courier, monospace;
        text-shadow: 0 0 10px rgba(0, 255, 157, 0.5);
    }
    
    /* 側邊欄背景 */
    [data-testid="stSidebar"] {
        background-color: #1F2937;
        border-right: 1px solid #00D2FF;
    }
    
    /* 按鈕樣式 (Primary) */
    .stButton > button {
        background-color: #00FF9D;
        color: #111827;
        border: 1px solid #00FF9D;
        border-radius: 5px;
        font-weight: bold;
        transition: all 0.3s ease;
    }
    .stButton > button:hover {
        background-color: #111827;
        color: #00FF9D;
        box-shadow: 0 0 15px #00FF9D;
    }

    /* 下載按鈕樣式 */
    .stDownloadButton > button {
        background-color: #00D2FF;
        color: #111827;
        border: 1px solid #00D2FF;
        font-weight: bold;
    }
    .stDownloadButton > button:hover {
        box-shadow: 0 0 15px #00D2FF;
    }
    
    /* Warning 文字 */
    .warning-text {
        color: #FF0055;
        font-weight: bold;
        animation: blinker 1.5s linear infinite;
    }
    @keyframes blinker {
        50% { opacity: 0.5; }
    }
    
    /* Dataframe 樣式調整 */
    [data-testid="stDataFrame"] {
        border: 1px solid #00D2FF;
    }
</style>
"""
st.markdown(CYBERPUNK_CSS, unsafe_allow_html=True)

# -----------------------------------------------------------------------------
# 核心邏輯函數
# -----------------------------------------------------------------------------

def parse_roc_date(date_str):
    """
    解析日期，支援標準格式與民國年 (ROC) 格式 (e.g. 112/01/01)
    """
    if pd.isna(date_str):
        return pd.NaT

    s = str(date_str).strip()

    # 1. 嘗試標準轉換
    try:
        return pd.to_datetime(s)
    except:
        pass

    # 2. 嘗試民國年 (ROC) 格式: 112/01/01 或 112-01-01
    # 簡單啟發式: 分隔符號 / 或 -
    parts = s.replace('-', '/').split('/')
    if len(parts) == 3:
        try:
            y, m, d = int(parts[0]), int(parts[1]), int(parts[2])
            # 民國年通常是 2 或 3 位數 (e.g. 99, 100, 112)
            # 轉換為西元: +1911
            if y < 1911:
                y += 1911
            return pd.Timestamp(year=y, month=m, day=d)
        except:
            pass

    return pd.NaT

@st.cache_data(show_spinner=False)
def get_whois_info(ip_address):
    """查詢 IP Whois 資訊 (使用 st.cache_data 快取結果)"""
    if not ip_address or ip_address == "N/A" or ip_address == "Invalid Data":
        return "", ""
    
    # 取第一個 IP
    first_ip = ip_address.split("|")[0].split(":")[-1].strip()
    
    try:
        url = f"http://ip-api.com/json/{first_ip}?fields=country,isp"
        response = requests.get(url, timeout=3)
        if response.status_code == 200:
            data = response.json()
            # 簡單限流
            time.sleep(0.5) 
            return data.get("country", "Unknown"), data.get("isp", "Unknown")
    except:
        pass
    
    return "Error", "Error"

def process_analysis(file_a, file_b, hide_sensitive, split_io, do_ip_match, do_whois):
    """執行分析主邏輯"""
    status_log = []
    
    try:
        # Load Data
        df_a = pd.read_excel(file_a)
        df_b = pd.read_excel(file_b)
        status_log.append(f"✅ 檔案載入成功: A({len(df_a)}筆), B({len(df_b)}筆)")

        # Prepare Split Dataframes (Before Drop)
        df_income = pd.DataFrame()
        df_expense = pd.DataFrame()
        
        if split_io:
            try:
                # 確保欄位足夠 (至少10欄)
                if df_a.shape[1] > 9:
                    # Force cleanup of 'J' (index 9) and 'I' (index 8)
                    val_inc = pd.to_numeric(df_a.iloc[:, 9], errors='coerce').fillna(0)
                    val_exp = pd.to_numeric(df_a.iloc[:, 8], errors='coerce').fillna(0)
                    
                    df_income = df_a[val_inc > 0].copy()
                    df_expense = df_a[val_exp > 0].copy()
                    status_log.append(f"📊 收支分流完成: 存入 {len(df_income)} 筆, 支出 {len(df_expense)} 筆")
                else:
                    status_log.append("⚠️ 收支分流失敗: 欄位不足 (需 >= 10)")
            except Exception as e:
                status_log.append(f"⚠️ 收支分流錯誤: {str(e)}")

        # Extract Counterparty Accounts (Assuming Index 5 / Column F)
        # Requirement: "需要多一個功能能把收支分流兩表的對方帳號，列出來並去重複"
        df_counterparty_list = pd.DataFrame()
        counterparty_col_idx = 5

        try:
            accs = set()
            # If split enabled, use split DFs
            if split_io:
                if not df_income.empty and df_income.shape[1] > counterparty_col_idx:
                     accs.update(df_income.iloc[:, counterparty_col_idx].dropna().astype(str).tolist())
                if not df_expense.empty and df_expense.shape[1] > counterparty_col_idx:
                     accs.update(df_expense.iloc[:, counterparty_col_idx].dropna().astype(str).tolist())
            else:
                # Fallback to df_a if split not enabled but user wants analysis?
                # Or just use df_a (which contains all)
                if df_a.shape[1] > counterparty_col_idx:
                    accs.update(df_a.iloc[:, counterparty_col_idx].dropna().astype(str).tolist())

            if accs:
                df_counterparty_list = pd.DataFrame(sorted(list(accs)), columns=['Unique_Counterparty_Account'])
                status_log.append(f"📋 對方帳號提取完成 ({len(df_counterparty_list)} 筆)")
        except Exception as e:
             status_log.append(f"⚠️ 對方帳號提取失敗: {str(e)}")

        # Drop Sensitive (Index 2, 5, 11, 12)
        if hide_sensitive:
            cols_to_drop = [2, 5, 11, 12]
            valid_cols = [c for c in cols_to_drop if c < df_a.shape[1]]
            if valid_cols:
                col_names = df_a.columns[valid_cols]

                # 同步隱藏分流表中的敏感欄位
                if not df_income.empty:
                    valid_drop_inc = [c for c in col_names if c in df_income.columns]
                    df_income.drop(columns=valid_drop_inc, inplace=True)

                if not df_expense.empty:
                    valid_drop_exp = [c for c in col_names if c in df_expense.columns]
                    df_expense.drop(columns=valid_drop_exp, inplace=True)

                # Drop inplace is safe here as income/expense are copies
                df_a.drop(columns=col_names, inplace=True)
                status_log.append(f"🛡️ 敏感欄位已隱藏 (Cols: {valid_cols})")

        # IP Matching
        if do_ip_match:
            status_log.append("🔄 正在執行 IP 交叉比對 (Window: -1s/+2s)...")
            
            # Pre-process File B for speed
            # Assume Col 0=Time, Col 1=Account, Col 2=IP
            # Safe check
            if df_b.shape[1] < 3:
                 status_log.append("❌ IP比對失敗: 檔案 B 欄位不足 (需 >= 3)")
            else:
                df_b_proc = df_b.iloc[:, [0, 1, 2]].copy()
                df_b_proc.columns = ['Time', 'Account', 'IP']
                df_b_proc['Time'] = pd.to_datetime(df_b_proc['Time'], errors='coerce')
                df_b_proc.dropna(subset=['Time'], inplace=True)
                
                # File A columns (Index 0, 1 assumed safely exist even after drop)
                # Note: df_a might have dropped cols, but typically 0,1 are not dropped (2,5,11,12)
                # times_a = pd.to_datetime(df_a.iloc[:, 0], errors='coerce')

                # 使用增強版日期解析 (支援 ROC)
                times_a = df_a.iloc[:, 0].apply(parse_roc_date)
                accs_a = df_a.iloc[:, 1]
                
                results = []
                
                # Note: Using Streamlit progress bar in main loop
                progress_bar = st.progress(0)
                total_rows = len(df_a)
                
                for idx in range(total_rows):
                    # Update progress every 10%
                    if idx % (max(1, total_rows // 10)) == 0:
                        progress_bar.progress(int((idx / total_rows) * 100))
                        
                    t = times_a.iloc[idx]
                    acc = accs_a.iloc[idx]
                    
                    if pd.isna(t) or pd.isna(acc):
                        results.append("Invalid Data")
                        continue
                    
                    # Window
                    t_start = t - datetime.timedelta(seconds=1)
                    t_end = t + datetime.timedelta(seconds=2)
                    
                    # Filter (Simple boolean mask)
                    mask = (df_b_proc['Account'] == acc) & \
                           (df_b_proc['Time'] >= t_start) & \
                           (df_b_proc['Time'] <= t_end)
                    
                    matches = df_b_proc[mask]
                    
                    if matches.empty:
                        results.append("N/A")
                    else:
                        ips = matches['IP'].unique()
                        if len(ips) == 1:
                            results.append(str(ips[0]))
                        else:
                            matches_list = []
                            for _, row in matches.iterrows():
                                delta = (row['Time'] - t).total_seconds()
                                sign = "+" if delta >= 0 else ""
                                matches_list.append(f"{sign}{int(delta)}s:{row['IP']}")
                            results.append(" | ".join(sorted(list(set(matches_list)))))
                
                progress_bar.progress(100)
                df_a['Matched_IP'] = results
                status_log.append(f"✅ IP 比對完成")

        # Whois
        if do_whois and 'Matched_IP' in df_a.columns:
            status_log.append("🌍 正在執行 Whois 線上反查...")
            unique_ips = df_a['Matched_IP'].unique()
            
            # Progress bar for Whois
            wb_bar = st.progress(0)
            
            countries = []
            isps = []
            
            for i, ip_text in enumerate(df_a['Matched_IP']):
                if i % (max(1, len(df_a) // 10)) == 0:
                    wb_bar.progress(int((i / len(df_a)) * 100))
                
                c, isp = get_whois_info(ip_text)
                countries.append(c)
                isps.append(isp)
                
            wb_bar.progress(100)
            df_a['IP_Country'] = countries
            df_a['IP_ISP'] = isps
            status_log.append("✅ Whois 查詢完成")

        # Generate Output
        output = io.BytesIO()
        with pd.ExcelWriter(output, engine='openpyxl') as writer:
            df_a.to_excel(writer, sheet_name='Sheet1_Summary', index=False)
            if split_io:
                df_income.to_excel(writer, sheet_name='Sheet2_Income', index=False)
                df_expense.to_excel(writer, sheet_name='Sheet3_Expense', index=False)

            if not df_counterparty_list.empty:
                df_counterparty_list.to_excel(writer, sheet_name='Sheet4_Counterparty', index=False)
        
        output.seek(0)
        return output, df_a, df_counterparty_list, status_log

    except Exception as e:
        status_log.append(f"❌ 嚴重錯誤: {str(e)}")
        return None, None, pd.DataFrame(), status_log

# -----------------------------------------------------------------------------
# UI 佈局
# -----------------------------------------------------------------------------

def main():
    st.title("🏦 BankFlow Tactical Analyzer")
    st.markdown("### 數位鑑識戰術分析系統 (Web Ver.)")
    st.divider()

    # --- Sidebar: 控制面板 ---
    st.sidebar.header("⚙️ Tactical Config")
    
    st.sidebar.markdown("---")
    sw_hide_sensitive = st.sidebar.toggle("隱藏敏感欄位 (C, F, L, M)", value=False)
    sw_split_io = st.sidebar.toggle("收支分流 (獨立 Sheet)", value=True)
    sw_ip_match = st.sidebar.toggle("IP 交叉比對 (±2s)", value=True)
    
    st.sidebar.markdown("---")
    st.sidebar.markdown('<span style="color:#FF0055">⚠️ OpSec Zone</span>', unsafe_allow_html=True)
    sw_whois = st.sidebar.toggle("Whois 線上反查", value=False)
    if sw_whois:
        st.sidebar.markdown('<p class="warning-text">警告: 將連線至外部 API</p>', unsafe_allow_html=True)

    # --- Main: 檔案輸入 ---
    col1, col2 = st.columns(2)
    
    with col1:
        st.subheader("📂 檔案 A (存款明細)")
        file_a = st.file_uploader("Upload Transaction Excel", type=["xls", "xlsx"], key="file_a")
        
    with col2:
        st.subheader("📂 檔案 B (IP 紀錄)")
        file_b = st.file_uploader("Upload IP Log Excel", type=["xls", "xlsx"], key="file_b")

    # --- Action: 執行分析 ---
    st.markdown("---")
    
    if st.button("🚀 EXECUTE ANALYSIS", use_container_width=True):
        if file_a and file_b:
            with st.spinner("SYSTEM PROCESSING..."):
                excel_data, result_df, cp_df, logs = process_analysis(
                    file_a, file_b, 
                    sw_hide_sensitive, sw_split_io, 
                    sw_ip_match, sw_whois
                )
            
            # 顯示 Logs
            with st.expander("System Logs", expanded=True):
                for log in logs:
                    st.text(log)

            if excel_data:
                st.balloons()
                st.success("分析完成! Target Neutralized.")
                
                # 下載按鈕
                timestamp = datetime.datetime.now().strftime("%Y%m%d_%H%M%S")
                st.download_button(
                    label="💾 下載分析報告 (.xlsx)",
                    data=excel_data,
                    file_name=f"Analysis_Result_{timestamp}.xlsx",
                    mime="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
                    use_container_width=True
                )
                
                # 顯示對方帳號列表
                if not cp_df.empty:
                    with st.expander(f"📋 對方帳號清單 (Total: {len(cp_df)})"):
                        st.dataframe(cp_df, use_container_width=True)

                # 數據預覽
                st.subheader("🔍 Result Preview (Top 10)")
                st.dataframe(result_df.head(10), use_container_width=True)

        else:
            st.error("請先載入兩個必要的 Excel 檔案 (File A & File B)")

if __name__ == "__main__":
    main()

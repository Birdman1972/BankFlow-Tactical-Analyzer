import pandas as pd
import os

def create_excel(path, headers):
    df = pd.DataFrame(columns=headers)
    df.to_excel(path, index=False)

base_dir = "fixtures/batch_test"
os.makedirs(base_dir, exist_ok=True)

# 2025-01: Standard Pair
os.makedirs(f"{base_dir}/2025-01", exist_ok=True)
create_excel(f"{base_dir}/2025-01/Report_A.xlsx", ["交易時間", "帳號", "支出金額", "存入金額"])
create_excel(f"{base_dir}/2025-01/Report_B.xlsx", ["登入時間", "帳號", "IP位址"])

# 2025-02: Alias Name Pair
os.makedirs(f"{base_dir}/2025-02", exist_ok=True)
create_excel(f"{base_dir}/2025-02/Account_Data.xlsx", ["交易時間", "帳號", "支出金額", "存入金額"])
create_excel(f"{base_dir}/2025-02/IP_Log_Data.xlsx", ["登入時間", "帳號", "IP位址"])

# 2025-03: Incomplete (A only)
os.makedirs(f"{base_dir}/2025-03", exist_ok=True)
create_excel(f"{base_dir}/2025-03/Report_A.xlsx", ["交易時間", "帳號", "支出金額", "存入金額"])

# 2025-04: Ambiguous (Multiple A's)
os.makedirs(f"{base_dir}/2025-04", exist_ok=True)
create_excel(f"{base_dir}/2025-04/Account_v1.xlsx", ["交易時間", "帳號", "支出金額", "存入金額"])
create_excel(f"{base_dir}/2025-04/Account_v2.xlsx", ["交易時間", "帳號", "支出金額", "存入金額"])
create_excel(f"{base_dir}/2025-04/IP_Log.xlsx", ["登入時間", "帳號", "IP位址"])

print("Batch fixtures created successfully.")

#!/usr/bin/env python3
"""
BankFlow 金流分析器 - 測試檔案產生器
產生符合格式要求的測試用 Excel 檔案
"""

import random
from datetime import datetime, timedelta
from openpyxl import Workbook
from pathlib import Path


def generate_account():
    """產生 10 位數銀行帳號"""
    return f"{random.randint(1000000000, 9999999999)}"


def generate_ip():
    """產生隨機 IP 位址"""
    return f"{random.randint(1, 223)}.{random.randint(0, 255)}.{random.randint(0, 255)}.{random.randint(1, 254)}"


def generate_transactions(count: int, accounts: list) -> list:
    """
    產生交易明細資料

    欄位對應:
    A (0): 交易時間
    B (1): 帳號
    C (2): 敏感資料 (身分證字號)
    D (3): 戶名
    E (4): 幣別
    F (5): 敏感資料 (交易序號)
    G (6): 摘要
    H (7): 備註
    I (8): 支出金額
    J (9): 存入金額
    K (10): 餘額
    L (11): 敏感資料 (對方帳號)
    M (12): 敏感資料 (對方戶名)
    """
    transactions = []
    base_time = datetime(2024, 1, 1, 8, 0, 0)

    summaries = [
        "轉帳",
        "提款",
        "存款",
        "匯款",
        "自動扣繳",
        "利息",
        "跨行轉入",
        "跨行轉出",
        "ATM",
        "網銀",
    ]

    for i in range(count):
        # 隨機時間 (2024-01-01 ~ 2024-01-31)
        time_offset = timedelta(
            days=random.randint(0, 30),
            hours=random.randint(0, 15),
            minutes=random.randint(0, 59),
            seconds=random.randint(0, 59),
        )
        timestamp = base_time + time_offset

        account = random.choice(accounts)

        # 決定存入或支出 (約60%存入, 40%支出)
        is_income = random.random() < 0.6

        if is_income:
            income = round(random.uniform(100, 500000), 0)
            expense = None
        else:
            income = None
            expense = round(random.uniform(100, 200000), 0)

        row = [
            timestamp.strftime("%Y/%m/%d %H:%M:%S"),  # A: 交易時間
            account,  # B: 帳號
            f"A{random.randint(100000000, 299999999)}",  # C: 身分證字號 (敏感)
            f"測試用戶{i % 100:03d}",  # D: 戶名
            "TWD",  # E: 幣別
            f"TX{random.randint(10000000, 99999999)}",  # F: 交易序號 (敏感)
            random.choice(summaries),  # G: 摘要
            f"備註{i % 50:02d}" if random.random() < 0.3 else "",  # H: 備註
            expense if expense else "",  # I: 支出金額
            income if income else "",  # J: 存入金額
            round(random.uniform(10000, 10000000), 0),  # K: 餘額
            f"{random.randint(1000000000, 9999999999)}"
            if random.random() < 0.7
            else "",  # L: 對方帳號 (敏感)
            f"對方戶名{random.randint(1, 500):03d}"
            if random.random() < 0.5
            else "",  # M: 對方戶名 (敏感)
        ]
        transactions.append((timestamp, account, row))

    # 按時間排序
    transactions.sort(key=lambda x: x[0])
    return transactions


def generate_ip_records(transactions: list, match_rate: float = 0.7) -> list:
    """
    產生 IP 登入紀錄

    欄位對應:
    A (0): 登入時間
    B (1): 帳號
    C (2): IP 位址

    參數:
    - transactions: 交易資料 (用於產生匹配的 IP 紀錄)
    - match_rate: 匹配率 (0.7 = 70% 交易會有匹配的 IP)
    """
    ip_records = []

    # 為每個帳號分配一些常用 IP
    account_ips = {}

    for timestamp, account, row in transactions:
        if account not in account_ips:
            # 每個帳號有 1-3 個常用 IP
            account_ips[account] = [generate_ip() for _ in range(random.randint(1, 3))]

        if random.random() < match_rate:
            # 產生匹配的 IP 紀錄 (在時間窗口 [-1s, +2s] 內)
            time_offset = random.uniform(-1, 2)
            ip_timestamp = timestamp + timedelta(seconds=time_offset)

            # 決定是單 IP 還是多 IP
            if random.random() < 0.15:  # 15% 機率是多 IP
                # 多 IP: 產生 2-3 條相近時間的紀錄
                num_ips = random.randint(2, 3)
                for j in range(num_ips):
                    extra_offset = random.uniform(-0.5, 0.5)
                    multi_timestamp = ip_timestamp + timedelta(seconds=extra_offset)
                    ip_records.append(
                        [
                            multi_timestamp.strftime("%Y/%m/%d %H:%M:%S"),
                            account,
                            random.choice(account_ips[account])
                            if j == 0
                            else generate_ip(),
                        ]
                    )
            else:
                # 單 IP
                ip_records.append(
                    [
                        ip_timestamp.strftime("%Y/%m/%d %H:%M:%S"),
                        account,
                        random.choice(account_ips[account]),
                    ]
                )

    # 加入一些不匹配的 IP 紀錄 (測試 N/A 情況)
    for _ in range(int(len(transactions) * 0.2)):
        random_timestamp = datetime(2024, 1, 1) + timedelta(
            days=random.randint(0, 30),
            hours=random.randint(0, 23),
            minutes=random.randint(0, 59),
            seconds=random.randint(0, 59),
        )
        random_account = f"{random.randint(1000000000, 9999999999)}"
        ip_records.append(
            [
                random_timestamp.strftime("%Y/%m/%d %H:%M:%S"),
                random_account,
                generate_ip(),
            ]
        )

    # 按時間排序
    ip_records.sort(key=lambda x: x[0])
    return ip_records


def create_excel_files(output_dir: Path, transaction_count: int = 1000):
    """建立測試用 Excel 檔案"""

    # Ensure reproducible fixtures
    random.seed(0)

    output_dir.mkdir(parents=True, exist_ok=True)

    # 產生固定的測試帳號 (10 個)
    accounts = [generate_account() for _ in range(10)]

    print(f"產生 {transaction_count} 筆交易資料...")
    transactions = generate_transactions(transaction_count, accounts)

    print(f"產生 IP 紀錄...")
    ip_records = generate_ip_records(transactions)

    # 建立檔案 A: 存款明細
    print("建立檔案 A: 存款明細...")
    wb_a = Workbook()
    ws_a = wb_a.active
    assert ws_a is not None
    ws_a.title = "存款明細"

    # 表頭
    headers_a = [
        "交易時間",
        "帳號",
        "身分證字號",
        "戶名",
        "幣別",
        "交易序號",
        "摘要",
        "備註",
        "支出",
        "存入",
        "餘額",
        "對方帳號",
        "對方戶名",
    ]
    ws_a.append(headers_a)

    # 資料
    for _, _, row in transactions:
        ws_a.append(row)

    file_a = output_dir / "test_transactions_1000.xlsx"
    wb_a.save(file_a)
    print(f"  已儲存: {file_a}")

    # 建立檔案 B: IP 紀錄
    print("建立檔案 B: IP 紀錄...")
    wb_b = Workbook()
    ws_b = wb_b.active
    assert ws_b is not None
    ws_b.title = "IP紀錄"

    # 表頭
    headers_b = ["登入時間", "帳號", "IP位址"]
    ws_b.append(headers_b)

    # 資料
    for row in ip_records:
        ws_b.append(row)

    file_b = output_dir / "test_ip_records_1000.xlsx"
    wb_b.save(file_b)
    print(f"  已儲存: {file_b}")

    # 統計
    print("\n" + "=" * 50)
    print("測試檔案產生完成!")
    print("=" * 50)
    print(f"交易筆數: {len(transactions)}")
    print(f"IP 紀錄筆數: {len(ip_records)}")
    print(f"測試帳號數: {len(accounts)}")
    print(f"\n測試帳號列表:")
    for acc in accounts:
        print(f"  - {acc}")

    return file_a, file_b


if __name__ == "__main__":
    # 設定輸出目錄
    script_dir = Path(__file__).parent
    output_dir = script_dir

    # 產生測試檔案
    create_excel_files(output_dir, transaction_count=1000)

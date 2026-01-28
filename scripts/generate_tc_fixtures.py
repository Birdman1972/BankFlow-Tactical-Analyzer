#!/usr/bin/env python3
"""
Generate rigorous BankFlow test fixtures with Traditional Chinese content.
"""
import argparse
import io
import math
import os
import random
from datetime import datetime, timedelta
from typing import List, Tuple

from openpyxl import Workbook

# --- Constants & Data Pools ---

FILE_A_HEADERS = [
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

FILE_B_HEADERS = [
    "登入時間",
    "帳號",
    "IP位址",
]

# Traditional Chinese Last Names
LAST_NAMES = ["陳", "林", "黃", "張", "李", "王", "吳", "劉", "蔡", "楊", "許", "鄭", "謝", "郭", "洪", "曾", "邱", "廖", "賴", "徐"]

# Traditional Chinese First Names (2 characters)
FIRST_NAMES = [
    "豪", "志明", "美玲", "淑芬", "俊傑", "雅婷", "冠宇", "宗翰", "家豪", "佳穎",
    "欣怡", "心怡", "志偉", "承恩", "宜君", "雅雯", "建宏", "怡君", "淑惠", "美惠",
    "佩君", "惠君", "嘉玲", "郁婷", "詩涵", "建志", "俊宏", "承翰", "冠志", "宇軒"
]

# Summaries (Transaction Types)
SUMMARIES = ["轉帳", "跨行提款", "現金存入", "薪資轉帳", "網購扣款", "一般消費", "代繳水費", "代繳電費", "信用卡款", "利息"]

# Remarks (Context)
REMARKS = [
    "網拍轉帳", "生活費", "還款", "聚餐", "買書", "繳房租", "不明", "測試備註", "蝦皮購物", "PChome",
    "UberEats", "Foodpanda", "自動扣繳", "手續費", "", "", "", ""  # Mix in some empty strings
]

PUBLIC_IPS = [
    "210.200.1.1", "220.135.10.10", "111.235.1.1", "59.124.1.1",  # TW IPs
    "8.8.8.8", "1.1.1.1", "140.112.1.1"
]

PRIVATE_IPS = [
    "192.168.1.10", "192.168.0.100", "10.0.0.5", "172.16.1.1"
]

# --- Generators ---

def random_name() -> str:
    return random.choice(LAST_NAMES) + random.choice(FIRST_NAMES)

def random_id(n: int) -> str:
    # Fake ID generator (not valid checksum, just format)
    first_char = chr(ord('A') + (n % 26))
    nums = f"{n % 1000000000:09d}"
    return f"{first_char}{nums}"

def build_file_a(row_count: int, seed: int = 42) -> bytes:
    random.seed(seed)
    wb = Workbook(write_only=True)
    ws = wb.create_sheet()
    ws.append(FILE_A_HEADERS)

def generate_shared_data(row_count: int, seed: int = 42) -> List[dict]:
    """Pre-calculate the core matching data (time, account) to ensure 100% parity."""
    random.seed(seed)
    base_time = datetime(2025, 4, 1, 9, 0, 0)
    accounts = [f"{i:012d}" for i in range(min(100, row_count))]
    
    data = []
    for i in range(row_count):
        # We use a NEW random instance here specifically for the time offset 
        # so it doesn't get rattled by a/b specific random calls
        offset = i * 60 + (i * 7) % 50 # Deterministic jitter without using global random
        data.append({
            "time": base_time + timedelta(seconds=offset),
            "account": accounts[i % len(accounts)]
        })
    return data

def build_file_a(row_count: int, shared_data: List[dict], seed: int = 42) -> bytes:
    random.seed(seed + 1) # Different seed for content
    wb = Workbook(write_only=True)
    ws = wb.create_sheet()
    ws.append(FILE_A_HEADERS)

    balance = 500000.0
    for i in range(row_count):
        sd = shared_data[i]
        timestamp = sd["time"].strftime("%Y/%m/%d %H:%M:%S")
        account = sd["account"]

        is_income = random.choice([True, False])
        income = 0.0
        expense = 0.0
        summary = random.choice(SUMMARIES)
        remark = random.choice(REMARKS)

        if is_income:
            income = float(random.randint(1, 500) * 100)
            balance += income
            if "扣" in summary or "費" in summary: summary = "現金存入"
        else:
            expense = float(random.randint(1, 200) * 100)
            balance -= expense
            if "存" in summary or "薪" in summary: summary = "一般消費"

        row = [
            timestamp, account, random_id(i), random_name(), "TWD",
            f"TXN{i:08d}", summary, remark, 
            expense if expense > 0 else "", 
            income if income > 0 else "",
            round(balance, 2), "", ""
        ]
        ws.append(row)

    buf = io.BytesIO()
    wb.save(buf)
    return buf.getvalue()

def build_file_b(row_count: int, shared_data: List[dict], seed: int = 42) -> bytes:
    random.seed(seed + 2)
    wb = Workbook(write_only=True)
    ws = wb.create_sheet()
    ws.append(FILE_B_HEADERS)

    # 1. Ensure 1:1 Match for EVERY transaction
    for sd in shared_data:
        ws.append([
            sd["time"].strftime("%Y/%m/%d %H:%M:%S"),
            sd["account"],
            random.choice(PUBLIC_IPS)
        ])

    # 2. Add Multi-IP and Noise
    remaining = row_count - len(shared_data)
    if remaining > 0:
        for j in range(remaining):
            # 50% chance of being a multi-ip match for an existing tx
            if random.random() < 0.5:
                sd = random.choice(shared_data)
                # within +2s window
                match_time = sd["time"] + timedelta(seconds=1)
                ws.append([
                    match_time.strftime("%Y/%m/%d %H:%M:%S"),
                    sd["account"],
                    random.choice(PRIVATE_IPS)
                ])
            else:
                # Random noise
                ws.append([
                    (datetime(2025, 4, 1, 9, 0, 0) + timedelta(seconds=random.randint(0, 86400))).strftime("%Y/%m/%d %H:%M:%S"),
                    random.choice([sd["account"] for sd in shared_data]),
                    random.choice(PUBLIC_IPS)
                ])

    buf = io.BytesIO()
    wb.save(buf)
    return buf.getvalue()

def write_file(path: str, data: bytes) -> None:
    path_obj = os.path.dirname(path)
    if path_obj: os.makedirs(path_obj, exist_ok=True)
    with open(path, "wb") as f: f.write(data)

def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--size", choices=["small", "large"], required=True)
    parser.add_argument("--out-prefix", required=True)
    args = parser.parse_args()
    
    rows_a, rows_b = (200, 300) if args.size == "small" else (5000, 8000)
    print(f"Generating {args.size} fixtures...")
    
    shared = generate_shared_data(rows_a)
    
    write_file(f"{args.out_prefix}_A.xlsx", build_file_a(rows_a, shared))
    write_file(f"{args.out_prefix}_B.xlsx", build_file_b(rows_b, shared))
    print("Done.")

if __name__ == "__main__":
    main()

if __name__ == "__main__":
    main()

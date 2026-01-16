
import pandas as pd
import datetime

def generate_test_data():
    print("Generating Test Data...")

    # --- 1. Generate Transaction Data (test_transaction.xlsx) ---
    # Structure: 13 columns (Col A to Col M)
    # A(0): Time (ROC), B(1): Account, C(2): Sensitive, D(3): Type, E(4): Memo
    # F(5): Counterparty (Sensitive), G(6): Bank, H(7): Branch
    # I(8): Expense, J(9): Income, K(10): Balance
    # L(11): Sensitive, M(12): Sensitive

    data_a = {
        '交易日期': ['112/01/01', '112/01/02', '112/01/03', '112/01/04'],
        '帳號': ['MyAccount001'] * 4,
        '身分證字號(C)': ['A123456789'] * 4, # Sensitive
        '交易代號': ['D001', 'D002', 'D003', 'D004'],
        '摘要': ['Salary', 'Shopping', 'Transfer', 'Utility'],
        '對方帳號(F)': ['CompanyAcc_A', 'ShopAcc_B', 'FriendAcc_C', 'WaterCo_D'], # Sensitive
        '對方銀行': ['BankA', 'BankB', 'BankC', 'BankD'],
        '分行': ['BranchA', 'BranchB', 'BranchC', 'BranchD'],
        '支出(I)': [0, 5000, 0, 1000], # Expense
        '存入(J)': [50000, 0, 3000, 0], # Income
        '結餘': [50000, 45000, 48000, 47000],
        '備註(L)': ['SecretL1', 'SecretL2', 'SecretL3', 'SecretL4'], # Sensitive
        '經辦(M)': ['UserM1', 'UserM2', 'UserM3', 'UserM4'] # Sensitive
    }
    df_a = pd.DataFrame(data_a)
    df_a.to_excel('test_transaction.xlsx', index=False)
    print("Generated: test_transaction.xlsx")

    # --- 2. Generate IP Log Data (test_ip_log.xlsx) ---
    # Structure: Time, Account, IP
    # Time must match the parsed transaction time (2023-01-01 etc.)
    # Window: -1s to +2s. We will set exact matches for simplicity.

    data_b = {
        '登入時間': [
            '2023-01-01 00:00:00', # Matches 112/01/01
            '2023-01-02 00:00:00', # Matches 112/01/02
            '2023-01-04 00:00:01', # Matches 112/01/04 (within +2s)
        ],
        '帳號': ['MyAccount001'] * 3,
        '來源IP': ['1.1.1.1', '2.2.2.2', '4.4.4.4']
    }
    df_b = pd.DataFrame(data_b)
    df_b.to_excel('test_ip_log.xlsx', index=False)
    print("Generated: test_ip_log.xlsx")

if __name__ == "__main__":
    generate_test_data()

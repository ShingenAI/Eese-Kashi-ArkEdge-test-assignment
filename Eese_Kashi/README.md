# ArkEdge Coding Test – Battery Depletion & Packet Parser

Submitted by: Eese Kashi
Language: Rust
Date: 2025-09-18

---

## Problem 1 – Battery Depletion

This program calculates the number of minutes until a satellite battery first depletes to zero, based on:
- `a`: Power generation during sunlight (W)
- `b`: Constant power consumption (W)
- `c`: Initial battery capacity (Wh)

The solution uses precise cycle-based analysis to detect:
- Immediate depletion during sun or shadow
- Long-term depletion over multiple 120-minute sun/shadow cycles
- Edge cases where depletion never occurs

**Input Format:**  
Single line with 3 space-separated integers, e.g.

40 30 20


**Usage:**
```bash
cargo run

It will read inputs from:

samples/sample1.txt
samples/sample2.txt

## Problem 2 – Binary Packet Parser
This program parses a binary stream containing multiple packets of the following format:

| 分類    | 名称   | 型                 | バイト数       | 内容           |
| ------ | ------ | ------------------ | ------------ | ------------- |
| HEADER | STX    | uint16_t           | 2            |  0xEB90       |
|        | LENGTH | uint16_t           | 2            | BODY のバイト数 |
| BODY   | BODY   | uint8_t x LENGTH   | LENGTH       | 任意データ      |
| FOOTER | ETX    | uint16_t           | 2 bytes      | 0xC579        |

The parser:

Validates STX/ETX and LENGTH bounds

Prints valid BODY content in hex

Skips malformed or partial packets gracefully

Usage:

Place binary input files in /samples/

Files used: sample1.dat, sample2.dat

Example Output:
::::::: Packet parser started :::::::

Path: samples/sample1.dat

Found STX at byte offset 0
Packet length = 5 bytes
  ✅ Valid packet! BODY contents:
0x12 0x34 0x56 0x78 0x9a 

::::::: Packet parser ended :::::::
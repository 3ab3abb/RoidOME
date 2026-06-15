#!/usr/bin/env python3
"""
RoidOME — ESP32-CAM Serial Frame Receiver
Reads base64-encoded JPEG frames from ESP32-CAM over serial
and saves them as JPEG files.

Usage:
    python3 receiver_frame.py
    python3 receiver_frame.py --port /dev/cu.usbserial-XXXX --baud 115200
"""

import serial
import serial.tools.list_ports
import base64
import os
import argparse
from datetime import datetime


# ── Config ────────────────────────────────────────────────────────────────────
DEFAULT_BAUD   = 115200
OUTPUT_DIR     = "frames"
# ──────────────────────────────────────────────────────────────────────────────


def find_esp32_port() -> str:
    """Auto-detect the ESP32 serial port."""
    ports = serial.tools.list_ports.comports()

    # common ESP32 identifiers
    esp_keywords = ["usbserial", "usbmodem", "cp210", "ch340", "ftdi", "esp32"]

    for port in ports:
        desc = (port.description or "").lower()
        name = (port.device or "").lower()
        if any(k in desc or k in name for k in esp_keywords):
            print(f"Auto-detected port: {port.device} ({port.description})")
            return port.device

    # fallback — list all ports and let user pick
    if ports:
        print("\nAvailable serial ports:")
        for i, p in enumerate(ports):
            print(f"  [{i}] {p.device} — {p.description}")
        choice = input("Select port number: ").strip()
        return ports[int(choice)].device

    raise RuntimeError("No serial ports found. Is the ESP32-CAM connected?")


def is_base64(s: str) -> bool:
    """Quick check — is this string a valid base64 payload?"""
    s = s.strip()
    # base64 strings are long and only contain valid chars
    if len(s) < 100:
        return False
    valid_chars = set("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=")
    return all(c in valid_chars for c in s)


def save_frame(data: bytes, output_dir: str) -> str:
    """Save JPEG bytes to disk with a timestamp filename."""
    os.makedirs(output_dir, exist_ok=True)
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S_%f")
    path = os.path.join(output_dir, f"frame_{timestamp}.jpg")
    with open(path, "wb") as f:
        f.write(data)
    return path


def receive_frames(port: str, baud: int, output_dir: str):
    """Main loop — read serial, decode frames, save to disk."""
    print(f"\nConnecting to {port} at {baud} baud...")
    print(f"Saving frames to: {os.path.abspath(output_dir)}/")
    print("Press Ctrl+C to stop\n")

    ser = serial.Serial(port, baud, timeout=10)
    frame_count = 0

    try:
        while True:
            # read one line from serial
            try:
                raw = ser.readline().decode("utf-8", errors="ignore").strip()
            except UnicodeDecodeError:
                continue

            if not raw:
                continue

            # print all non-base64 lines (debug output from ESP32)
            if not is_base64(raw):
                print(f"[ESP32] {raw}")
                continue

            # it's a base64 frame — decode and save
            print(f"[FRAME] Received {len(raw)} chars — decoding...", end=" ")

            try:
                # strip any whitespace before decoding
                cleaned = raw.replace(" ", "").replace("\n", "").replace("\r", "")
                jpeg_bytes = base64.b64decode(cleaned)

                # verify it's actually a JPEG (starts with FF D8 FF)
                if jpeg_bytes[:2] != b'\xff\xd8':
                    print("✗ Not a valid JPEG — skipping")
                    continue

                path = save_frame(jpeg_bytes, output_dir)
                frame_count += 1
                print(f"✓ Saved {len(jpeg_bytes)} bytes → {path}")

                # open in Preview automatically on Mac
                os.system(f"open '{path}'")

            except Exception as e:
                print(f"✗ Failed: {e}")

    except KeyboardInterrupt:
        print(f"\nStopped. Received {frame_count} frame(s).")
    finally:
        ser.close()


def main():
    parser = argparse.ArgumentParser(description="ESP32-CAM Serial Frame Receiver")
    parser.add_argument("--port", "-p", help="Serial port (auto-detected if omitted)")
    parser.add_argument("--baud", "-b", type=int, default=DEFAULT_BAUD,
                        help=f"Baud rate (default: {DEFAULT_BAUD})")
    parser.add_argument("--output", "-o", default=OUTPUT_DIR,
                        help=f"Output directory (default: {OUTPUT_DIR})")
    args = parser.parse_args()

    port = args.port or find_esp32_port()
    receive_frames(port, args.baud, args.output)


if __name__ == "__main__":
    main()

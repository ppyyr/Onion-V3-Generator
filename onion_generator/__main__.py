import base64
import hashlib
import threading
import multiprocessing
import time
import sys
import select
import os
import queue
import signal
from cryptography.hazmat.primitives.asymmetric import ed25519
from cryptography.hazmat.primitives.serialization import Encoding, PrivateFormat, NoEncryption, PublicFormat
import argparse

class OnionGenerator:
    @staticmethod
    def increment_generated():
        # This will be handled by the main process
        pass

    @staticmethod
    def increment_found():
        # This will be handled by the main process
        pass

    @staticmethod
    def generate_address():
        # Generate key pair
        private_key = ed25519.Ed25519PrivateKey.generate()
        public_key = private_key.public_key()
        
        # Serialize keys
        private_bytes = private_key.private_bytes(Encoding.Raw, PrivateFormat.Raw, NoEncryption())
        public_bytes = public_key.public_bytes(Encoding.Raw, PublicFormat.Raw)
        
        # Expand secret key
        expanded_secret_key = OnionGenerator.expand_secret_key(private_bytes)
        
        # Generate onion address
        onion_address = OnionGenerator.encode_public_key(public_bytes)
        
        public = b"== ed25519v1-public: type0 ==\x00\x00\x00" + public_bytes
        secret = b"== ed25519v1-secret: type0 ==\x00\x00\x00" + expanded_secret_key
        
        # Count will be handled by main process
        
        return {
            "hostname": onion_address,
            "public": base64.b64encode(public).decode(),
            "private": base64.b64encode(secret).decode()
        }


    @staticmethod
    def encode_public_key(public_key):
        checksum = hashlib.sha3_256(b".onion checksum" + public_key + b"\x03").digest()[:2]
        onion_address = public_key + checksum + b"\x03"
        return OnionGenerator.base32_encode(onion_address).lower() + ".onion"

    @staticmethod
    def expand_secret_key(secret_key):
        # Expand the secret key to meet the required format
        hash_bytes = hashlib.sha512(secret_key).digest()
        hash_list = list(hash_bytes)
        hash_list[0] &= 248
        hash_list[31] &= 127
        hash_list[31] |= 64
        
        return bytes(hash_list)

    @staticmethod
    def base32_encode(data):
        # Base32 encode the data
        return base64.b32encode(data).decode('utf-8').strip("=")




def worker_process(prefixes, result_queue, stats_queue, process_id):
    """Worker function for multi-process address generation"""
    local_generated = 0

    # Set up signal handler for graceful shutdown
    def signal_handler(signum, frame):
        if local_generated > 0:
            stats_queue.put(('update', local_generated, 0))
        sys.exit(0)

    signal.signal(signal.SIGTERM, signal_handler)
    signal.signal(signal.SIGINT, signal_handler)

    try:
        while True:
            addr = OnionGenerator.generate_address()
            local_generated += 1

            for prefix in prefixes:
                if addr["hostname"].startswith(prefix):
                    # Send stats update and result
                    stats_queue.put(('update', local_generated, 1))
                    result_queue.put(addr)
                    local_generated = 0  # Reset counter after reporting
                    break

            # Periodically update stats even if no match found
            if local_generated % 1000 == 0:
                stats_queue.put(('update', local_generated, 0))
                local_generated = 0

    except (KeyboardInterrupt, SystemExit):
        if local_generated > 0:
            stats_queue.put(('update', local_generated, 0))
        sys.exit(0)

def stats_monitor(stats_queue, stop_event):
    """Monitor and update statistics from worker processes"""
    generated = 0
    found = 0

    while not stop_event.is_set():
        try:
            msg_type, gen_count, found_count = stats_queue.get(timeout=1)
            if msg_type == 'update':
                generated += gen_count
                found += found_count
        except:
            continue

    return generated, found

def periodic_update(interval, stats_queue, stop_event):
    """Print periodic status updates"""
    generated = 0
    found = 0

    while not stop_event.is_set():
        time.sleep(interval)

        # Collect all pending stats updates
        while True:
            try:
                msg_type, gen_count, found_count = stats_queue.get_nowait()
                if msg_type == 'update':
                    generated += gen_count
                    found += found_count
            except:
                break

        print(f"[@] {time.strftime('%H:%M:%S')}: Generated {generated} addresses, Found {found} addresses")

def keypress_update(stats_queue, stop_event):
    """Handle keypress for manual status updates"""
    if not sys.stdin.isatty():
        print("[!] Non-TTY environment detected. Keypress updates are disabled.\n")
        return

    print("[i] Press Enter to see the current status:\n")
    generated = 0
    found = 0

    while not stop_event.is_set():
        if select.select([sys.stdin], [], [], 0.1)[0]:
            sys.stdin.read(1)

            # Collect all pending stats updates
            while True:
                try:
                    msg_type, gen_count, found_count = stats_queue.get_nowait()
                    if msg_type == 'update':
                        generated += gen_count
                        found += found_count
                except:
                    break

            print(f"[@] {time.strftime('%H:%M:%S')}: Generated {generated} addresses, Found {found} addresses")


def main():
    parser = argparse.ArgumentParser(description="Generate Tor .onion addresses.")
    parser.add_argument("prefixes", nargs="+", help="List of prefixes for the hostname")
    parser.add_argument("--threads", type=int, default=None,
                        help="Number of threads to use (default: CPU core count)")
    args = parser.parse_args()

    # Ensure at least one prefix is provided
    if not args.prefixes:
        print("[!] Error: At least one prefix must be provided.")
        sys.exit(1)

    prefixes = [prefix.strip().lower() for prefix in args.prefixes]

    # Determine number of processes
    if args.threads is not None:
        num_processes = max(1, args.threads)  # Ensure at least 1 process
    else:
        num_processes = os.cpu_count() or 4  # Default to CPU core count

    print(f"[@] Using {num_processes} processes for address generation")

    # Create multiprocessing queues and events
    result_queue = multiprocessing.Queue()
    stats_queue = multiprocessing.Queue()
    stop_event = threading.Event()

    # Start monitoring threads
    threading.Thread(target=periodic_update, args=(30, stats_queue, stop_event), daemon=True).start()
    threading.Thread(target=keypress_update, args=(stats_queue, stop_event), daemon=True).start()

    # Start worker processes
    worker_processes = []
    for i in range(num_processes):
        p = multiprocessing.Process(
            target=worker_process,
            args=(prefixes, result_queue, stats_queue, i)
        )
        p.start()
        worker_processes.append(p)
        print(f"[@] Started worker process {i+1} with PID {p.pid}")

    try:
        print("[@] Generating addresses...")
        while True:
            # Get result from worker threads (blocking call, no timeout)
            result = result_queue.get()

            print('[√] Address generated successfully!')
            print(f"Hostname:                      {result['hostname']}")
            print(f"Public Key (Base64 encoded):   {result['public']}")
            print(f"Private Key (Base64 encoded):  {result['private']}\n")

    except KeyboardInterrupt:
        print("[!] Stopping generation...")
        stop_event.set()

        # Terminate all worker processes
        for p in worker_processes:
            p.terminate()

        # Wait for processes to terminate
        for p in worker_processes:
            p.join(timeout=2)
            if p.is_alive():
                p.kill()  # Force kill if still alive

        print("[!] All processes stopped.")


if __name__ == "__main__":
    # Use fork method on Unix systems for better compatibility
    if hasattr(os, 'fork'):
        multiprocessing.set_start_method('fork', force=True)
    main()

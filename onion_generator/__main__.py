import base64
import hashlib
import nacl.signing
import nacl.encoding
import argparse


class OnionGenerator:
    @staticmethod
    def generate_address():
        # Generate key pair
        signing_key = nacl.signing.SigningKey.generate()
        secret_key = signing_key.encode()
        expanded_secret_key = OnionGenerator.expand_secret_key(secret_key)
        public_key = signing_key.verify_key.encode()
        
        # Generate onion address
        onion_address = OnionGenerator.encode_public_key(public_key)
        
        public = b"== ed25519v1-public: type0 ==\x00\x00\x00" + public_key
        secret = b"== ed25519v1-secret: type0 ==\x00\x00\x00" + expanded_secret_key
        
        return {
            "hostname": onion_address,
            "public": base64.b64encode(public).decode(),
            "private": base64.b64encode(secret).decode()
        }


    @staticmethod
    def generate_with_prefix(prefix):
        while True:
            generated = OnionGenerator.generate_address()
            if generated["hostname"].startswith(prefix):
                return generated


    @staticmethod
    def encode_public_key(public_key):
        checksum = hashlib.sha3_256(b".onion checksum" + public_key + b"\x03").digest()[:2]
        onion_address = public_key + checksum + b"\x03"
        return OnionGenerator.base32_encode(onion_address).lower() + ".onion"


    @staticmethod
    def expand_secret_key(secret_key):
        # Expand the secret key to meet the required format
        hash_bytes = hashlib.sha512(secret_key[:nacl.bindings.crypto_box_SECRETKEYBYTES]).digest()
        hash_list = list(hash_bytes)
        hash_list[0] &= 248
        hash_list[31] &= 127
        hash_list[31] |= 64
        
        return bytes(hash_list)


    @staticmethod
    def base32_encode(data):
        # Base32 encode the data
        return base64.b32encode(data).decode('utf-8').strip("=")


def main():
    parser = argparse.ArgumentParser(description="Generate Tor .onion addresses.")
    parser.add_argument("--prefix", type=str, help="Prefix for the hostname", default=None)
    parser.add_argument("--count", type=int, help="Number of addresses to generate (-1 for infinite)", default=1)
    args = parser.parse_args()

    generated_count = 0

    try:
        print("[@] Generating addresses...\n")
        while args.count == -1 or generated_count < args.count:
            if args.prefix:
                result = OnionGenerator.generate_with_prefix(args.prefix)
            else:
                result = OnionGenerator.generate_address()

            print('[√] Address generated successfully!')
            print(f"Hostname:                      {result['hostname']}")
            print(f"Public Key (Base64 encoded):   {result['public']}")
            print(f"Private Key (Base64 encoded):  {result['private']}\n")

            generated_count += 1
    except KeyboardInterrupt:
        print("[!] Stopping generation...")


if __name__ == "__main__":
    main()

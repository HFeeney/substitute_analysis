This program performs decryption of english language ciphertext encrypted with the subsitution method on alphabetic characters. It reads ciphertext from stdin, outputting its best guess for the decrypted plaintext to stdout.

Example usage:
cat medium_encrypted.txt | cargo run

Limitations of this program include poor performance on small ciphertexts, or ciphertexts with abnormal english language (i.e. different character/bigram/trigram frequencies than normal english text).

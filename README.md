## Rust CLI Password Manager

This project provides a secure and user-friendly command-line interface (CLI) password manager written in Rust. It empowers you to effectively manage your passwords without relying on third-party services.

Written in rust and using different crates like clap (cli management) and <Decriptoin crate>.

## Features

* **Master Password Protection:** A single, strong master password acts as the key to decrypt and access your password vault.
* **Password Management:**
    * **Addition:** Create new password entries associated with an specific alias.
    * **Retrieval:** Securely access the password for a specific alias.
    * **Editing:** Update existing passwords when necessary.
    * **Deletion:** Remove password entries you no longer require.
* **Clear and Concise CLI:** Intuitive commands facilitate seamless interaction with the password manager.

## TODO

* **Robust Encryption:** Leverages industry-standard encryption algorithms (e.g., AES-256) to safeguard your passwords, ensuring they remain unreadable even in case of unauthorized access.
* **Secure Random Password Generation:** Generates strong, random passwords to enhance account security.

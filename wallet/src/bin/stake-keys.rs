use std::env;
use wallet::RecoveryBuilder;

fn main() {
    let mut mnemonics = String::new();
    let mut args = env::args();
    args.next();
    for argument in args {
        mnemonics.push_str(&argument);
        mnemonics.push(' ');
    }

    println!("{}", mnemonics);

    let builder = RecoveryBuilder::new();

    let builder = builder
        .mnemonics(&bip39::dictionary::ENGLISH, mnemonics.as_str().trim_end())
        .unwrap();

    let icarus = builder.build_yoroi().unwrap();

    for account in icarus.stake_accounts() {
        let xprv = account.private_key();

        println!("{}", xprv.display());
    }
}

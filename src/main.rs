struct BankAccount {
  balance: i32,
  verified: bool
}

fn print_balance(account: &BankAccount) {
  println!("{:?}", account.balance);
}

fn print_verified(account: &BankAccount) {
  println!("{:?}", account.verified);
}

fn is_verfied(account: &BankAccount) -> Result<bool, bool> {
  return match account.verified {
    true => Ok(true),
    false => Err(false)
  }
}

fn main() {
  let my_account = BankAccount {
    balance: 230,
    verified: false
  };
  print_balance(&my_account);
  print_verified(&my_account);

  // let verification_status = is_verfied(&my_account).unwrap();
  let verification_status = is_verfied(&my_account).expect("An error happened");

  print!("{:?}", verification_status);
}

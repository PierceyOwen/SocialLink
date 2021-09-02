mod accounts;
fn main() {
   let accounts = accounts::get_active_accounts(); 
    println!("{}",accounts);
}


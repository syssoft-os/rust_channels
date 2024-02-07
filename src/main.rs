use rand::Rng;
use std::env;
use std::sync::{mpsc,Arc,Mutex};
use std::thread;

fn create_random_array(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut array = Vec::with_capacity(n);
    for _ in 0..n {
        array.push(rng.gen_range(0..10001));
    }
    array
}

struct Transfer {
    acc1: usize,
    acc2: usize,
    amount: i32,
}

fn transfer(array: &mut Vec<i32>, transfer: Transfer) {
    if array[transfer.acc1] >= transfer.amount {
        array[transfer.acc1] -= transfer.amount;
        array[transfer.acc2] += transfer.amount;
        // println!("Transfered {} from account {} to account {}", transfer.amount, transfer.acc1, transfer.acc2);
    }
    else {
        // println!("Transfer from account {} to account {} failed", transfer.acc1, transfer.acc2);
    }
}

fn get_argument ( a:&String, comment:&str ) -> usize {
    let n: usize = match a.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("{}",comment);
            return 0;
        }
    };
    n
}

fn main() {
    println!("Rust Bank - The safest place for your money!");
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        println!("Arguments: <number of accounts> <number of server threads> <number of clients> <number of transfers>");
        return;
    }
    let n = get_argument(&args[1], "Please provide a valid number of accounts");
    let n_servers = get_argument(&args[2], "Please provide a valid number of server threads");
    let n_clients = get_argument(&args[3], "Please provide a valid number of clients");
    let n_transfers = get_argument(&args[4], "Please provide a valid number of transfers");
    println!("Configuration: {} accounts, {} server threads, {} clients with {} transfers each", n, n_servers, n_clients, n_transfers);

    let mut accounts = create_random_array(n);

    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));

    let mut servers = vec![];

    for _ in 0..n_servers {
        let rx = rx.clone();
        let server = thread::spawn(move || {
            for received in rx.lock().unwrap().iter() {
                transfer(&mut accounts, received);
            }
        });
        servers.push(server);
    }

    let mut clients = vec![];

    for _ in 0..n_clients {
        let tx = tx.clone();
        let client = thread::spawn(move || {
            let mut rng = rand::thread_rng();
            for _ in 0..n_transfers {
                let acc1 = rng.gen_range(0..n);
                let acc2 = rng.gen_range(0..n);
                let amount = rng.gen_range(1..5000);
                // println!("Client transferring {} from account {} to account {}", amount, acc1, acc2);
                tx.send(Transfer { acc1, acc2, amount }).unwrap();
            }
        });
        clients.push(client);
    }

    for client in clients {
        client.join().unwrap();
    }

    drop(tx);

    for server in servers {
        server.join().unwrap();
    }

}

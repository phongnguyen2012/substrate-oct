mod step1;
mod step2;
mod step3;
mod step4;
mod step5;
mod step6;

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_step1() {
    let mut balances = step1::BalancesModule::new();

    balances.set_balance(1, 500);
    balances.set_balance(2, 200);
    balances.transfer(1, 2, 80).unwrap();
    assert_eq!(balances.get_balance(1), &420);
    assert_eq!(balances.get_balance(2), &280);
}

#[test]
fn test_step2() {
    let mut balances = step2::BalancesModule::new();

    balances.set_balance(1, 500);
    balances.set_balance(2, 300);
    balances.transfer(1, 2, 100).unwrap();
    assert_eq!(balances.get_balance(1), &400);
    assert_eq!(balances.get_balance(2), &400);
}
#[test]
fn test_step3() {
    let mut voting = step3::VotingModule::new();

    voting.vote(1, 10, true);
    voting.vote(2, 10, false);

    assert_eq!(voting.get_vote(1, 10), true);
    assert_eq!(voting.get_vote(2, 10), false);
}

#[test]
fn test_step3_1() {
    let user1 = 1;
    let user2 = 2;
    let mut balances = step2::BalancesModule::new();
    let mut voting = step3::VotingModule::new();
    //test transfer
    balances.set_balance(user1, 500);
    balances.set_balance(user2, 300);

    balances.transfer(user1, user2, 100).unwrap();
    assert_eq!(balances.get_balance(user1), &400);
    assert_eq!(balances.get_balance(user2), &400);

    //test vote
    let user_a = 1u64;
    let user_b = 2u64;
    voting.vote(user_a, 10, true);
    voting.vote(user_b, 10, false);

    assert_eq!(voting.get_vote(user_a, 10), true);
    assert_eq!(voting.get_vote(user_b, 10), false);

}
#[test]
fn test_step4a() {
    let acc1 = 1;
    let acc2 = 2;
    let mut balances = step4::BalancesModule::<u32,u64>::new();

    balances.set_balance(acc1, 900);
    balances.set_balance(acc2, 200);
    balances.transfer(acc1, acc2, 100).unwrap();
    assert_eq!(balances.get_balance(acc1), 800);
    assert_eq!(balances.get_balance(acc2), 300);
}
#[test]
fn test_step4b() {
    let acc1 = 1;
    let acc2 = 2;
    let mut balances = step4::BalancesModule::<u16,u64>::new();
    
    balances.set_balance(acc1, 900);
    balances.set_balance(acc2, 200);
    balances.transfer(acc1, acc2, 100).unwrap();
    assert_eq!(balances.get_balance(acc1), 800);
    assert_eq!(balances.get_balance(acc2), 300);

    //test 2

    let mut balan = step4::BalancesModule::<String,u64>::new();

    let a = String::from("a");
    let b = String::from("b");

    balan.set_balance(a.clone(), 900);
    balan.set_balance(b.clone(), 200);
    balan.transfer(a.clone(), b.clone(), 100).unwrap();
    assert_eq!(balan.get_balance(a), 800);
    assert_eq!(balan.get_balance(b), 300);
}
#[test]
fn test_step4c() {
    struct Runtime;
    impl step6::Config for Runtime {
        type AccountId = u64;
        type VoteIndex = u64;
    } 
    let mut voting  = step6::VotingModule::<Runtime>::new();
    let user1 = 1u64;
    let user2 = 2u64;
    
    voting.vote(user1, 10, true);
    voting.vote(user2, 10, false);
    
}
#[test]
fn test_step5() {
    struct  TestRuntime;
    impl step5::Config for TestRuntime {
        type AccountId = u32;
        type VoteIndex= u32;
        type Balance = u64;
        
    }
    let mut balances = step5::BalancesModule::<TestRuntime>::new();
    let u1: <TestRuntime as step5::Config>::AccountId = 1;
    let u2: <TestRuntime as step5::Config>::AccountId = 2;
    balances.set_balance(u1, 500);
    balances.set_balance(u2, 200);
    balances.transfer(u1, u2, 80).unwrap();
    assert_eq!(balances.get_balance(u1), 420);
    assert_eq!(balances.get_balance(u2), 280);
}
#[test]
fn test_step5a() {
    struct  TestRuntime;
    impl step5::Config for TestRuntime {
        type AccountId = u32;
        type VoteIndex= u32;
        type Balance = u64;
    }
    let mut balances = step5::BalancesModule::<TestRuntime>::new();
    let u1: <TestRuntime as step5::Config>::AccountId = 1;
    let u2: <TestRuntime as step5::Config>::AccountId = 2;
    balances.set_balance(u1, 500);
    balances.set_balance(u2, 200);
    balances.transfer(u1, u2, 80).unwrap();
    assert_eq!(balances.get_balance(u1), 420);
    assert_eq!(balances.get_balance(u2), 280);
}
use super::*;

use crate as domainname;
use sp_core::H256;
use frame_support::{
    parameter_types, assert_ok, assert_noop,
};
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
		Domain: domainname::{Module, Call, Storage, Event<T>, Config},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl frame_system::Config for Test {
	type BaseCallFilter = ();
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type Balance = u64;
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

parameter_types! {
	pub const DefaultDifficulty: u32 = 3;
}

impl Config for Test {
    type Event = Event;
    type Currency = Balances;
    // type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();

    pallet_balances::GenesisConfig::<Test>{
		balances: vec![(200, 500), (201, 400), (202, 50)],
    }.assimilate_storage(&mut t).unwrap();

    let mut t: sp_io::TestExternalities = t.into();

    t.execute_with(|| System::set_block_number(1) );
    t
}

fn last_event() -> Event {
    System::events().last().unwrap().event.clone()
}

fn event_has_been_fired(expected_event: Event) -> bool {
    System::events().iter().any(|a| a.event == expected_event)
}

#[test]
fn it_can_claim_a_domain_name() {
	new_test_ext().execute_with(|| {
		let domain = b"janislav.eth";
		assert_ok!(Domain::register(Origin::signed(200), domain.to_vec()));
		let expected = Domain::domains(domain.to_vec());
		assert_eq!(expected, 200);
	});
}

#[test]
fn it_throws_an_not_found() {
	new_test_ext().execute_with(|| {
		let not_existing_domain = b"not_found.eth";
		let e = Error::<Test>::DomainNotFound;
		assert_noop!(Domain::send(Origin::signed(1), 50, not_existing_domain.to_vec()), e);
	});
}

#[test]
fn it_can_send_money_by_an_domain_name() {
	new_test_ext().execute_with(|| {
		let domain = b"janislav.eth";
		assert_ok!(Domain::register(Origin::signed(200), domain.to_vec()));
		assert_ok!(Domain::send(Origin::signed(201), 100, domain.to_vec()));
		assert_eq!(Balances::total_balance(&200), 600);
		assert_eq!(Balances::total_balance(&201), 300);
	});
}

#[test]
fn it_can_unregister_an_domain() {
	new_test_ext().execute_with(|| {
		let domain = b"janislav.eth";
		assert_ok!(Domain::register(Origin::signed(200), domain.to_vec()));
		//assert_eq!(last_event(), Event::kitties(RawEvent::KittySold(100, 200, 0, 400)));
		assert_eq!(last_event(), Event::domainname(RawEvent::Registered(domain.to_vec(), 200)));
		assert_ok!(Domain::unregister(Origin::signed(200), domain.to_vec()));
		assert_eq!(last_event(), Event::domainname(RawEvent::Unregistered(domain.to_vec(), 200)));
	});
}

#[test]
fn it_fails_if_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let domain = b"janislav.eth";
		assert_ok!(Domain::register(Origin::signed(200), domain.to_vec()));
		assert_noop!(Domain::send(Origin::signed(202), 100, domain.to_vec()), pallet_balances::Error::<Test>::InsufficientBalance);
	});
}
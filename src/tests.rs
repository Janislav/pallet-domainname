use crate::{Error, mock::*};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_can_claim_a_domain_name() {
	new_test_ext().execute_with(|| {
		let domain = b"janislav.eth";
		assert_ok!(Domain::register(Origin::signed(1), domain.to_vec()));
		let expected = Domain::domains(domain.to_vec());
		assert_eq!(expected, 1);
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

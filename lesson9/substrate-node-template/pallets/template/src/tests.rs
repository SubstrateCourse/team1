// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn test_onchain() {
	new_test_ext().execute_with(|| {
		// Test onchain logic here
	});
}

#[test]
fn test_offchain() {
	new_test_ext().execute_with(|| {
		// Test offchain worker here
	});
}

// Tests to be written here

use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

#[test]
fn create_claim_works() {
    // PoeModule::create_claim();
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), (1, system::Module::<Test>::block_number()));
    })
}

#[test]
fn create_claim_failure_when_exists() {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofAlreadyExist
        );
    })
}

#[test]
fn create_claim_failure_too_long() {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2,5,6,5,5,5,6,4,3];
        assert_noop!(
            PoeModule::create_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ProofTooLong
        );
    })
}

#[test]
fn revoke_claim_works () {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
    })
}

#[test]
fn revoke_claim_failed_if_not_exists () {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];

        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
            Error::<Test>::ClaimNotExist);
    })
}

#[test]
fn revoke_claim_failed_if_not_claim_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
            Error::<Test>::NotClaimOwner);
    })
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
    })
}

#[test]
fn transfer_claim_failed_if_not_claim_owner() {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 2),
            Error::<Test>::NotClaimOwner);
    })
}

#[test]
fn revoke_claim_failed_if_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![1,2];
        //let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(
            PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
            Error::<Test>::ClaimNotExist);
    })
}
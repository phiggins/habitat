// Copyright (c) 2016 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use protocol::vault;
use vault::data_store::DataStore;

#[test]
fn migration() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
    });
}

#[test]
fn create_origin() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        let mut origin = vault::OriginCreate::new();
        origin.set_name(String::from("neurosis"));
        origin.set_owner_id(1);
        origin.set_owner_name(String::from("scottkelly"));
        ds.create_origin(&origin).expect("Should create origin");
    });
}

#[test]
fn get_origin_by_name() {
    with_pool!(pool, {
        let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
        ds.setup().expect("Failed to migrate data");
        let mut origin = vault::OriginCreate::new();
        origin.set_name(String::from("neurosis"));
        origin.set_owner_id(1);
        origin.set_owner_name(String::from("scottkelly"));
        ds.create_origin(&origin).expect("Should create origin");

        let new_origin = ds.get_origin_by_name("neurosis").expect("Could not get the origin");
        assert!(new_origin.is_some(), "Origin did not exist");
        let fg = new_origin.unwrap();
        assert_eq!(fg.get_name(), "neurosis");
        assert_eq!(fg.get_owner_id(), 1);
        assert_eq!(fg.get_private_key_name(), "");
    });
}

#[test]
fn create_origin_secret_key() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = vault::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin secret key
    let mut oskc = vault::OriginSecretKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_secret").into_bytes());
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");

    // Origin secret keys get returned with the origin
    let first = ds.get_origin_by_name("neurosis")
        .expect("Could not get the origin")
        .expect("origin did not exist");
    assert_eq!(first.get_private_key_name(), "neurosis-20160612031944");

    // They are also sorted based on the latest key if there is more than one
    oskc.set_revision(String::from("20160612031945"));
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");
    let second = ds.get_origin_by_name("neurosis")
        .expect("Could not get the origin")
        .expect("origin did not exist");
    assert_eq!(second.get_private_key_name(), "neurosis-20160612031945");
}

#[test]
fn get_origin_secret_key() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = vault::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    // Create a new origin secret key
    let mut oskc = vault::OriginSecretKeyCreate::new();
    oskc.set_name(String::from("neurosis"));
    oskc.set_revision(String::from("20160612031944"));
    oskc.set_origin_id(neurosis.get_id());
    oskc.set_owner_id(1);
    oskc.set_body(String::from("very_secret").into_bytes());
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");
    oskc.set_revision(String::from("20160612031945"));
    ds.create_origin_secret_key(&oskc).expect("Failed to create origin secret key");

    let mut osk_get = vault::OriginSecretKeyGet::new();
    osk_get.set_origin(String::from("neurosis"));
    osk_get.set_owner_id(1);
    let neurosis_key = ds.get_origin_secret_key(&osk_get)
        .expect("Failed to get origin secret key from database")
        .expect("No origin secret key found in database");
    assert_eq!(neurosis_key.get_name(), "neurosis");
    assert_eq!(neurosis_key.get_revision(), "20160612031945");
    assert_eq!(neurosis_key.get_origin_id(), neurosis.get_id());
    assert_eq!(neurosis_key.get_body(), oskc.get_body());
    assert_eq!(neurosis_key.get_owner_id(), oskc.get_owner_id());
}

#[test]
fn create_origin_invitation() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = vault::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut oic = vault::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(2);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");
    ds.create_origin_invitation(&oic)
        .expect("Failed to create the origin invitation again, which should be a no-op");

    oic.set_owner_id(5);
    ds.create_origin_invitation(&oic)
        .expect("Failed to create the origin invitation again, which should be a no-op");

    // We should never create an invitation for the same person and org
    let conn = ds.pool.get().expect("Cannot get connection from pool");
    let rows = conn.query("SELECT COUNT(*) FROM origin_invitations", &[])
        .expect("Failed to query database for number of invitations");
    let count: i64 = rows.iter().nth(0).unwrap().get(0);
    assert_eq!(count, 1);
}

#[test]
fn list_origin_invitations_for_origin() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = vault::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut oilr = vault::OriginInvitationListRequest::new();
    oilr.set_origin_id(neurosis.get_id());
    let no_invites = ds.list_origin_invitations_for_origin(&oilr)
        .expect("Failed to get origin list from database");
    assert!(no_invites.is_none(),
            "We have invitations when we should have none");

    let mut oic = vault::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(2);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");
    oic.set_account_id(3);
    oic.set_account_name(String::from("maynard_james_keenan"));
    ds.create_origin_invitation(&oic)
        .expect("Failed to create the origin invitation");
    oic.set_account_id(4);
    oic.set_account_name(String::from("danny_cary"));
    ds.create_origin_invitation(&oic)
        .expect("Failed to create the origin invitation");

    // List comes back in alphabetical order by origin
    let oi_list = ds.list_origin_invitations_for_origin(&oilr)
        .expect("Could not get origin invitation list from database")
        .expect("No origin invites for origin that should have 3");
    assert_eq!(oi_list.len(), 3);
    let danny = oi_list.iter().nth(0).unwrap();
    assert_eq!(danny.get_account_id(), 4);
    let maynard = oi_list.iter().nth(1).unwrap();
    assert_eq!(maynard.get_account_id(), 3);
    let noel = oi_list.iter().nth(2).unwrap();
    assert_eq!(noel.get_account_id(), 2);
}

#[test]
fn list_origin_invitations_for_account() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = vault::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut torigin = vault::OriginCreate::new();
    torigin.set_name(String::from("tool"));
    torigin.set_owner_id(2);
    torigin.set_owner_name(String::from("maynard"));
    ds.create_origin(&torigin).expect("Should create origin");

    let tool = ds.get_origin_by_name("tool")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut ailr = vault::AccountInvitationListRequest::new();
    ailr.set_account_id(3);
    let no_invites = ds.list_origin_invitations_for_account(&ailr)
        .expect("Failed to get origin list from database");
    assert!(no_invites.is_none(),
            "We have invitations when we should have none");

    let mut oic = vault::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(3);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    oic.set_origin_id(tool.get_id());
    oic.set_origin_name(String::from(tool.get_name()));
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    // We shouldn't see mr pants in our result set
    oic.set_account_id(4);
    oic.set_account_name(String::from("poopy_pants"));
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    // List comes back in alphabetical order by origin
    let oi_list = ds.list_origin_invitations_for_account(&ailr)
        .expect("Could not get origin invitation list from database")
        .expect("No origin invites for origin that should have 3");
    assert_eq!(oi_list.len(), 2);

    let neurosis = oi_list.iter().nth(0).unwrap();
    assert_eq!(neurosis.get_origin_name(), "neurosis");
    let tool = oi_list.iter().nth(1).unwrap();
    assert_eq!(tool.get_origin_name(), "tool");
}

#[test]
fn accept_origin_invitation() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = vault::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut oic = vault::OriginInvitationCreate::new();
    oic.set_origin_id(neurosis.get_id());
    oic.set_origin_name(String::from(neurosis.get_name()));
    oic.set_account_id(3);
    oic.set_account_name(String::from("noel_gallagher"));
    oic.set_owner_id(1);
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");

    let mut ailr = vault::AccountInvitationListRequest::new();
    ailr.set_account_id(3);
    let invite = ds.list_origin_invitations_for_account(&ailr)
        .expect("failed to get invitations from database")
        .expect("there should be invitatations")
        .into_iter()
        .nth(0)
        .expect("there should be an invitation");

    let mut oiar = vault::OriginInvitationAcceptRequest::new();
    oiar.set_account_accepting_request(3);
    oiar.set_invite_id(invite.get_id() as u64);
    oiar.set_ignore(false);
    ds.accept_origin_invitation(&oiar).expect("Failed to accept origin invitation");

    // Accepting an invitation means deleting the invite from the table
    assert!(ds.list_origin_invitations_for_account(&ailr)
                .expect("Failed to get invitations from database")
                .is_none(),
            "Invitations were not deleted on acceptance");

    // Create the invitation again - it should not take, because the member exists
    ds.create_origin_invitation(&oic).expect("Failed to create the origin invitation");
    assert!(ds.list_origin_invitations_for_account(&ailr)
                .expect("Failed to get invitations from database")
                .is_none(),
            "Invitation was created even if the member exists");

    oic.set_account_id(4);
    oic.set_account_name(String::from("steve_perry"));
    ds.create_origin_invitation(&oic).expect("Failed to create an origin invitation");
    ailr.set_account_id(4);
    let steves_invite = ds.list_origin_invitations_for_account(&ailr)
        .expect("failed to get invitations from database")
        .expect("there should be invitatations")
        .into_iter()
        .nth(0)
        .expect("there should be an invitation");
    oiar.set_account_accepting_request(4);
    oiar.set_invite_id(steves_invite.get_id() as u64);
    oiar.set_ignore(true);
    ds.accept_origin_invitation(&oiar).expect("Failed to accept origin invitation");

    // Accepting with an ignore means you will never see this request
    assert!(ds.list_origin_invitations_for_account(&ailr)
                .expect("Failed to get invitations from database")
                .is_none(),
            "Ignored Invitations were not removed from the list on acceptance");

    let mut omlr = vault::OriginMemberListRequest::new();
    omlr.set_origin_id(neurosis.get_id());
    let members = ds.list_origin_members(&omlr)
        .expect("Failed to get origin members from the database")
        .expect("There should be members in the database");
    assert!(members.contains(&String::from("scottkelly")),
            "scotkelly should be a member");
    assert!(members.contains(&String::from("noel_gallagher")),
            "noel_gallagher should be a member");
    assert!(!members.contains(&String::from("steve_perry")),
            "steve_perry is a member, but he ignored his invite");
}

#[test]
fn check_account_in_origin() {
    let pool = pool!();
    let ds = DataStore::from_pool(pool).expect("Failed to create data store from pool");
    ds.setup().expect("Failed to migrate data");
    let mut origin = vault::OriginCreate::new();
    origin.set_name(String::from("neurosis"));
    origin.set_owner_id(1);
    origin.set_owner_name(String::from("scottkelly"));
    ds.create_origin(&origin).expect("Should create origin");

    let neurosis = ds.get_origin_by_name("neurosis")
        .expect("Could not retrieve origin")
        .expect("Origin does not exist");

    let mut coar = vault::CheckOriginAccessRequest::new();
    coar.set_origin_name(String::from("neurosis"));
    coar.set_account_id(1);

    assert!(ds.check_account_in_origin(&coar).expect("failed to check membership in the database"),
            "Member should have been in the origin");
}

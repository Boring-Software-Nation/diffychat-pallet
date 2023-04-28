use crate::{mock::*, Error, Event, ItemByAccountId};
use frame_support::{assert_noop, assert_ok};
use frame_system::ensure_signed;

#[test]
fn test_register() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let sender = RuntimeOrigin::signed(1);

		let sender_addr = ensure_signed(sender.clone()).unwrap();
		let nickname = [123_u8; 20];
		let address = [1_u8; 32];

		assert_ok!(TemplateModule::register(sender, nickname.clone(), address.clone()));

		let addr_resp = TemplateModule::get_address_by_nickname(nickname.clone());

		assert_eq!(address, addr_resp);

		let item_by_account_id = TemplateModule::get_address_by_account_id(sender_addr);

		assert_eq!(item_by_account_id, ItemByAccountId { address, nickname })
	})
}

#[test]
fn test_register_account_id_is_already_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let sender = RuntimeOrigin::signed(1);
		let nickname = [123_u8; 20];
		let address = [1_u8; 32];

		assert_ok!(TemplateModule::register(sender.clone(), nickname.clone(), address));

		let addr_resp = TemplateModule::get_address_by_nickname(nickname);

		assert_eq!(address, addr_resp);

		assert_noop!(
			TemplateModule::register(sender, nickname, address),
			Error::<Test>::AccountIdAlreadyRegistered,
		);
	})
}

#[test]
fn test_register_nickname_is_already_registered() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let sender = RuntimeOrigin::signed(1);
		let sender2 = RuntimeOrigin::signed(2);

		let nickname = [123_u8; 20];
		let address = [1_u8; 32];

		assert_ok!(TemplateModule::register(sender, nickname.clone(), address));

		let addr_resp = TemplateModule::get_address_by_nickname(nickname);

		assert_eq!(address, addr_resp);

		assert_noop!(
			TemplateModule::register(sender2, nickname, address),
			Error::<Test>::NicknameAlreadyRegistered,
		);
	})
}

#[test]
fn offer_chat_with_static_values() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let sender = RuntimeOrigin::signed(1);
		let receiver = RuntimeOrigin::signed(2);

		let offer = [3u8; 2048];
		let welcome_msg = [1u8; 300];

		let sender_account_id = ensure_signed(sender.clone()).expect("cant get account id");
		let receiver_account_id = ensure_signed(receiver).expect("cant get account id");

		assert_ok!(TemplateModule::offer_chat(
			sender,
			welcome_msg.clone(),
			offer.clone(),
			receiver_account_id,
		));

		System::assert_last_event(
			Event::Offer {
				offer,
				offered_by: sender_account_id,
				offered_to: receiver_account_id,
				welcome_msg,
			}
			.into(),
		)
	});
}

#[test]
fn answer_chat_with_static_values() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);

		let sender = RuntimeOrigin::signed(1);
		let receiver = RuntimeOrigin::signed(2);

		let answer = [3u8; 2048];

		let sender_account_id = ensure_signed(sender.clone()).expect("cant get account id");
		let receiver_account_id = ensure_signed(receiver).expect("cant get account id");

		assert_ok!(TemplateModule::answer_chat(sender, answer.clone(), receiver_account_id,));

		System::assert_last_event(
			Event::Answer {
				answer,
				answer_from: sender_account_id,
				answer_to: receiver_account_id,
			}
			.into(),
		)
	});
}

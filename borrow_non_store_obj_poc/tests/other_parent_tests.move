#[test_only]
module borrow_non_store_obj_poc::other_users_borrow_tests;

use borrow_non_store_obj_poc::other_parent::{Self, SomeOtherParent};
use iota::test_scenario as ts;
use tf_components::borrowed_child;
use tf_components::example_child::{Self, ExampleChildObject};

const ALICE: address = @0xA;
const MALLORY: address = @0xB;

/// Helper function to borrow an ExampleChildObject from SomeOtherParent
public fun borrow_example_child(
    parent_obj: &mut SomeOtherParent,
): (borrowed_child::Pledge, ExampleChildObject) {
    let child_receiving = ts::receiving_ticket_by_id<
        ExampleChildObject,
    >(ts::most_recent_id_for_address<ExampleChildObject>(object::id(
        parent_obj,
    ).to_address()).extract());
    borrowed_child::borrow_example(parent_obj.uid_mut(), child_receiving).extract_example()
}

#[test]
/// Test borrowing child, using it, and putting it back using SomeOtherParent instead of ExampleParentObject
fun test_borrow_child_and_put_back() {
    let mut scenario = ts::begin(ALICE);

    // Step 1: Create child object
    example_child::create(scenario.ctx());

    // Step 2: Create parent object
    scenario.next_tx(ALICE);
    other_parent::create(scenario.ctx());

    // Step 3: Transfer child to parent object
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        let parent_obj = scenario.take_shared<SomeOtherParent>();

        let parent_address = object::id(&parent_obj).to_address();
        example_child::transfer_object(child_obj, parent_address);

        ts::return_shared(parent_obj);
    };

    // Step 4: Borrow child, get counter, and put back
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_shared<SomeOtherParent>();

        // Borrow the child
        let (pledge, child) = borrow_example_child(&mut parent_obj);

        // Get counter (should be 0 initially)
        let counter = example_child::get_counter(&child);
        assert!(counter == 0, 0);

        // Put back the child
        borrowed_child::put_back_example(&object::id(&parent_obj), pledge, child);

        ts::return_shared(parent_obj);
    };

    scenario.end();
}

#[test]
/// Test stealing the child
fun steal_child() {
    let mut scenario = ts::begin(ALICE);

    // Step 1: ALICE creates a child object
    example_child::create(scenario.ctx());

    // Step 2: ALICE creates a parent object
    scenario.next_tx(ALICE);
    other_parent::create(scenario.ctx());

    // Step 3: Transfer child to parent object
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        let parent_obj = scenario.take_shared<SomeOtherParent>();

        let parent_address = object::id(&parent_obj).to_address();
        example_child::transfer_object(child_obj, parent_address);

        ts::return_shared(parent_obj);
    };

    // Step 4: Bob gets the child, gets the counter to prove access and steals in the end
    scenario.next_tx(MALLORY);
    {
        let mut parent_obj = scenario.take_shared<SomeOtherParent>();

        // Identify the latest child
        let child_receiving = ts::receiving_ticket_by_id<
            ExampleChildObject,
        >(ts::most_recent_id_for_address<ExampleChildObject>(object::id(
            &parent_obj,
        ).to_address()).extract());

        // MALLORY receives the child
        let child = example_child::receive(parent_obj.uid_mut(), child_receiving);

        // Get counter (should be 0 initially)
        let counter = child.get_counter();
        assert!(counter == 0, 0);

        example_child::transfer_object(child, MALLORY);

        ts::return_shared(parent_obj);
    };

    scenario.end();
}

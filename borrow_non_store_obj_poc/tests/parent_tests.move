#[test_only]
module borrow_non_store_obj_poc::parent_tests;

use iota::test_scenario as ts;

use tf_components::borrowed_child;
use tf_components::example_child::{Self, ExampleChildObject};
use borrow_non_store_obj_poc::parent::{Self, ExampleParentObject};

const ALICE: address = @0xA;

/// Helper function to borrow a
public fun borrow_example_child(
    parent_obj: &mut ExampleParentObject,
): (borrowed_child::Pledge, ExampleChildObject) {
    let child_receiving = ts::receiving_ticket_by_id<
        ExampleChildObject,
    >(ts::most_recent_id_for_address<ExampleChildObject>(object::id(
        parent_obj,
    ).to_address()).extract());
    parent_obj.borrow_child(borrowed_child::request_example(child_receiving)).extract_example()
}

#[test]
/// Test creating a parent object
fun test_create_parent() {
    let mut scenario = ts::begin(ALICE);

    // Create parent object
    parent::create(scenario.ctx());

    // Verify parent was created and transferred to sender
    scenario.next_tx(ALICE);
    {
        let parent_obj = scenario.take_from_sender<ExampleParentObject>();
        scenario.return_to_sender(parent_obj);
    };

    scenario.end();
}

#[test]
/// Test creating a child object
fun test_create_child() {
    let mut scenario = ts::begin(ALICE);

    // Create child object
    example_child::create(scenario.ctx());

    // Verify child was created and transferred to sender
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        assert!(example_child::get_counter(&child_obj) == 0, 0);
        scenario.return_to_sender(child_obj);
    };

    scenario.end();
}

#[test]
/// Test transferring child object to parent
fun test_transfer_child_to_parent() {
    let mut scenario = ts::begin(ALICE);

    // Step 1: Create child object
    example_child::create(scenario.ctx());

    // Step 2: Create parent object
    scenario.next_tx(ALICE);
    parent::create(scenario.ctx());

    // Step 3: Transfer child to parent object
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        let parent_obj = scenario.take_from_sender<ExampleParentObject>();

        // Transfer child to parent's address
        let parent_address = object::id(&parent_obj).to_address();
        example_child::transfer_object(child_obj, parent_address);

        scenario.return_to_sender(parent_obj);
    };

    scenario.end();
}

/*
#[test]
/// Test receiving child and incrementing counter
fun test_receive_increment_child() {
    let mut scenario = ts::begin(ALICE);
    
    // Step 1: Create child object
    example_child::create(scenario.ctx());
    
    // Step 2: Create parent object
    scenario.next_tx(ALICE);
    parent::create(scenario.ctx());
    
    // Step 3: Transfer child to parent object
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        let parent_obj = scenario.take_from_sender<ExampleParentObject>();
        
        let parent_address = object::id(&parent_obj).to_address();
        example_child::transfer_object(child_obj, parent_address);
        
        scenario.return_to_sender(parent_obj);
    };
    
    // Step 4: Receive child and increment counter
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_from_sender<ExampleParentObject>();
        let child_receiving = ts::receiving_ticket_by_id<ExampleChildObject>(
            ts::most_recent_id_for_address<ExampleChildObject>(object::id(&parent_obj).to_address()).extract()
        );
        
        parent::receive_increment_child(&mut parent_obj, child_receiving);
        
        scenario.return_to_sender(parent_obj);
    };
    
    scenario.end();
}
*/

#[test]
/// Test borrowing child, using it, and putting it back
fun test_borrow_child_and_put_back() {
    let mut scenario = ts::begin(ALICE);

    // Step 1: Create child object
    example_child::create(scenario.ctx());

    // Step 2: Create parent object
    scenario.next_tx(ALICE);
    parent::create(scenario.ctx());

    // Step 3: Transfer child to parent object
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        let parent_obj = scenario.take_from_sender<ExampleParentObject>();

        let parent_address = object::id(&parent_obj).to_address();
        example_child::transfer_object(child_obj, parent_address);

        scenario.return_to_sender(parent_obj);
    };

    // Step 4: Borrow child, get counter, and put back
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_from_sender<ExampleParentObject>();

        // Borrow the child
        let (pledge, child) = borrow_example_child(&mut parent_obj);

        // Get counter (should be 0 initially)
        let counter = example_child::get_counter(&child);
        assert!(counter == 0, 0);

        // Put back the child
        parent_obj.put_back(borrowed_child::example(pledge, child));

        scenario.return_to_sender(parent_obj);
    };

    scenario.end();
}

#[test]
/// Test the full flow: create, transfer, increment, borrow, verify counter
fun test_full_poc_flow() {
    let mut scenario = ts::begin(ALICE);

    // Step 1: Create child object
    example_child::create(scenario.ctx());

    // Step 2: Create parent object
    scenario.next_tx(ALICE);
    parent::create(scenario.ctx());

    // Step 3: Transfer child to parent object
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        let parent_obj = scenario.take_from_sender<ExampleParentObject>();

        let parent_address = object::id(&parent_obj).to_address();
        example_child::transfer_object(child_obj, parent_address);

        scenario.return_to_sender(parent_obj);
    };

    // Step 4: Borrow child, increment counter and verify counter was incremented
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_from_sender<ExampleParentObject>();

        // Borrow the child
        let (pledge, mut child) = borrow_example_child(&mut parent_obj);

        // Increment the counter
        child.increment();

        // Verify counter was incremented to 1
        let counter = child.get_counter();
        assert!(counter == 1, 0);

        // Put back the child
        parent_obj.put_back(borrowed_child::example(pledge, child));

        scenario.return_to_sender(parent_obj);
    };

    scenario.end();
}

#[test]
/// Test incrementing child counter multiple times
fun test_multiple_increments() {
    let mut scenario = ts::begin(ALICE);

    // Create child and parent
    example_child::create(scenario.ctx());
    scenario.next_tx(ALICE);
    parent::create(scenario.ctx());

    // Transfer child to parent
    scenario.next_tx(ALICE);
    {
        let child_obj = scenario.take_from_sender<ExampleChildObject>();
        let parent_obj = scenario.take_from_sender<ExampleParentObject>();

        let parent_address = object::id(&parent_obj).to_address();
        example_child::transfer_object(child_obj, parent_address);

        scenario.return_to_sender(parent_obj);
    };

    // First increment
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_from_sender<ExampleParentObject>();
        let (pledge, mut child) = borrow_example_child(&mut parent_obj);
        child.increment();
        parent_obj.put_back(borrowed_child::example(pledge, child));
        scenario.return_to_sender(parent_obj);
    };

    // Second increment
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_from_sender<ExampleParentObject>();
        let (pledge, mut child) = borrow_example_child(&mut parent_obj);
        child.increment();
        parent_obj.put_back(borrowed_child::example(pledge, child));
        scenario.return_to_sender(parent_obj);
    };

    // Third increment
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_from_sender<ExampleParentObject>();
        let (pledge, mut child) = borrow_example_child(&mut parent_obj);
        child.increment();
        parent_obj.put_back(borrowed_child::example(pledge, child));
        scenario.return_to_sender(parent_obj);
    };

    // Verify counter is 3
    scenario.next_tx(ALICE);
    {
        let mut parent_obj = scenario.take_from_sender<ExampleParentObject>();

        let (pledge, child) = borrow_example_child(&mut parent_obj);
        assert!(example_child::get_counter(&child) == 3, 0);

        parent_obj.put_back(borrowed_child::example(pledge, child));
        scenario.return_to_sender(parent_obj);
    };

    scenario.end();
}

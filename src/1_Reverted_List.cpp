#include <iostream>
#include <cassert>

// Definition of the ListNode structure
struct ListNode
{
    int value;      // Value of the node
    ListNode *next; // Pointer to the next node in the list

    // Constructor initializes the node with a value and sets next to nullptr
    ListNode(int val) : value(val), next(nullptr) {}
};

// Definition of the List class
struct List
{
    // Constructor initializes the list with an empty head
    List() : head(nullptr) {}

    // Destructor cleans up all nodes
    ~List()
    {
        clean();
    }

    // Inserts a new element at the beginning of the list
    void insert(int val)
    {
        ListNode *newNode = new ListNode(val);
        newNode->next = head;
        head = newNode;
    }

    // Reverses the list
    void revert()
    {
        ListNode *prev = nullptr;
        ListNode *current = head;
        ListNode *next = nullptr;

        // Traverse the list and reverse the links
        while (current != nullptr)
        {
            next = current->next;
            current->next = prev;
            prev = current;
            current = next;
        }

        // Reset the head to the new front of the list
        head = prev;
    }

    // Frees the memory allocated for the list
    void clean()
    {
        ListNode *current = head;
        ListNode *next = nullptr;

        // Traverse the list and delete each node
        while (current != nullptr)
        {
            next = current->next;
            delete current;
            current = next;
        }

        // Ensure the head is nullptr after cleaning
        head = nullptr;
    }

    // Prints the values in the list
    void printList()
    {
        ListNode *current = head;
        // Traverse the list and print each value
        while (current != nullptr)
        {
            std::cout << current->value << " ";
            current = current->next;
        }
        std::cout << std::endl;
    }

private:
    ListNode *head; // Pointer to the first node in the list
};

// Function for testing the List class
void testList()
{
    List lst;
    lst.insert(2);
    lst.insert(3);
    lst.insert(4);
    lst.insert(5);

    // Output the list and check the insertion order
    std::cout << "Original List: ";
    lst.printList();
    // Expected output: "Original List: 5 4 3 2 "

    lst.revert();

    // Output the reverted list and check the reverse order
    std::cout << "Reverted List: ";
    lst.printList();
    // Expected output: "Reverted List: 2 3 4 5 "

    // Clean up the list to free memory
    lst.clean();
}

int main()
{
    // Run the test
    testList();

    return 0;
}
#include <iostream>

// The myshared_ptr class template
template <typename T>
class myshared_ptr
{
public:
    // Constructor that takes a raw pointer and initializes the reference count
    explicit myshared_ptr(T *ptr = nullptr) : _ptr(ptr), _ref_count(new std::size_t(1))
    {
        std::cout << "myshared_ptr constructor called\n";
    }

    // Copy constructor that increases the reference count
    myshared_ptr(const myshared_ptr &m) : _ptr(m._ptr), _ref_count(m._ref_count)
    {
        ++*_ref_count;
        std::cout << "myshared_ptr copy constructor called\n";
    }

    // Copy assignment operator that handles self-assignment and adjusts reference counts
    myshared_ptr &operator=(const myshared_ptr &m)
    {
        // Protect against self-assignment
        if (this != &m)
        {
            // Decrease the old reference count
            if (--*_ref_count == 0)
            {
                delete _ptr;
                delete _ref_count;
            }
            // Copy the new pointer and new reference count
            _ptr = m._ptr;
            _ref_count = m._ref_count;
            ++*_ref_count;
        }
        std::cout << "myshared_ptr copy assignment called\n";
        return *this;
    }

    // Destructor that deletes the managed object if the reference count reaches zero
    ~myshared_ptr()
    {
        if (--*_ref_count == 0)
        {
            delete _ptr;
            delete _ref_count;
            std::cout << "myshared_ptr destructor called, object deleted\n";
        }
        else
        {
            std::cout << "myshared_ptr destructor called, object not deleted\n";
        }
    }

    // Overload dereference and arrow operators
    T &operator*() const { return *_ptr; }
    T *operator->() const { return _ptr; }

private:
    T *_ptr;                 // Pointer to the managed object
    std::size_t *_ref_count; // Pointer to the reference count
};

int main()
{
    // Create a myshared_ptr to an integer
    myshared_ptr<int> ptr1(new int(10));
    std::cout << "Value: " << *ptr1 << std::endl; // Output the value

    // Create another myshared_ptr using the copy constructor
    myshared_ptr<int> ptr2(ptr1);
    std::cout << "Value: " << *ptr2 << std::endl; // Output the value

    // Create a third myshared_ptr using the copy assignment operator
    myshared_ptr<int> ptr3;
    ptr3 = ptr1;
    std::cout << "Value: " << *ptr3 << std::endl; // Output the value

    // The destructors will be called automatically for ptr1, ptr2, and ptr3
    // when they go out of scope, and the integer will be deleted when the last
    // myshared_ptr is destroyed.

    return 0;
}
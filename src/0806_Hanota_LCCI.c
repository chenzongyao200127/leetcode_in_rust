// Define a structure for representing a frame in the custom stack.
typedef struct
{
    int pc;             // Program counter to track the next operation
    int n;              // Number of disks
    char from, to, via; // Identifiers for the rods (source, destination, auxiliary)
} Frame;

// Macro to simulate a function call, pushing a new frame onto the stack.
#define call(...) ({ *(++top) = (Frame){.pc = 0, __VA_ARGS__}; })

// Macro to simulate a function return, popping a frame from the stack.
#define ret() ({ top--; })

// Macro to simulate a goto operation, modifying the program counter of the current frame.
#define goto(loc) ({ f->pc = (loc)-1; })

// Function to solve the Tower of Hanoi problem iteratively.
void hanoi(int n, char from, char to, char via)
{
    Frame stk[64], *top = stk - 1; // Stack initialization with a fixed size and top pointer

    // Simulate the initial function call
    call(n, from, to, via);

    // Main loop for processing each frame on the stack
    for (Frame *f; (f = top) >= stk; f->pc++)
    {
        // Load the frame's context into local variables
        n = f->n;
        from = f->from;
        to = f->to;
        via = f->via;

        // Switch-case to determine the next action based on the program counter
        switch (f->pc)
        {
        case 0:
            if (n == 1) // Base case: move a single disk directly
            {
                printf("%c -> %c\n", from, to);
                goto(4); // Skip to the end of the function
            }
            break;
        case 1:
            call(n - 1, from, via, to); // Simulate recursive call to move n-1 disks
            break;
        case 2:
            call(1, from, to, via); // Move the remaining disk
            break;
        case 3:
            call(n - 1, via, to, from); // Move the n-1 disks to the target rod
            break;
        case 4:
            ret(); // Return from the function call
            break;
        default:
            assert(0); // Assert failure in case of an unexpected program counter value
        }
    }
}

#include <iostream>
#include <string>
#include <map>
#include <functional>

// FSM状态的类型
enum class State
{
    INIT,
    RUNNING,
    FINISHED,
    ERROR
    // 更多的状态可以在这里添加
};

// FSM输入事件的类型
enum class Event
{
    START,
    FINISH,
    FAIL
    // 更多的事件可以在这里添加
};

class FSM
{
private:
    // 当前状态
    State currentState;

    // 状态转换表类型定义
    using TransitionTable = std::map<std::pair<State, Event>, std::function<void()>>;

    // 状态转换表
    TransitionTable transitions;

    // 初始化状态转换表
    void setupTransitions()
    {
        // 初始状态转换
        transitions[{State::INIT, Event::START}] = [this]()
        {
            std::cout << "Transition from INIT to RUNNING" << std::endl;
            this->currentState = State::RUNNING;
            // TODO: 实现具体的状态转换逻辑
        };

        // 运行状态转换
        transitions[{State::RUNNING, Event::FINISH}] = [this]()
        {
            std::cout << "Transition from RUNNING to FINISHED" << std::endl;
            this->currentState = State::FINISHED;
            // TODO: 实现具体的状态转换逻辑
        };

        // 错误处理
        transitions[{State::RUNNING, Event::FAIL}] = [this]()
        {
            std::cout << "Transition from RUNNING to ERROR" << std::endl;
            this->currentState = State::ERROR;
            // TODO: 实现具体的错误处理逻辑
        };

        // 其他状态转换可以在这里添加
    }

public:
    // 构造函数初始化为初始状态
    FSM() : currentState(State::INIT)
    {
        setupTransitions();
    }

    // 事件处理函数，外部接口
    void handleEvent(Event event)
    {
        auto it = transitions.find({currentState, event});
        if (it != transitions.end())
        {
            // 找到了状态转换，执行对应的动作
            it->second();
        }
        else
        {
            // 未找到状态转换，可能需要处理异常
            std::cout << "Invalid transition" << std::endl;
            // TODO: 实现未找到状态转换时的处理逻辑
        }
    }

    // 获取当前状态，外部接口
    State getCurrentState() const
    {
        return currentState;
    }
};

int main()
{
    FSM fsm;

    // 示例代码，演示状态机的使用
    std::cout << "Current state: " << static_cast<int>(fsm.getCurrentState()) << std::endl;
    fsm.handleEvent(Event::START);
    std::cout << "Current state: " << static_cast<int>(fsm.getCurrentState()) << std::endl;
    fsm.handleEvent(Event::FINISH);
    std::cout << "Current state: " << static_cast<int>(fsm.getCurrentState()) << std::endl;

    return 0;
}
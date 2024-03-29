#include <iostream>
#include <vector>
#include <unordered_map>
#include <optional>

enum class MessageType
{
    M1 = 0,
    M2 = 1,
    M3 = 2,
    M4 = 3,
    M5 = 4,
};

using Priority = uint8_t;

const std::optional<Priority> getPriority(MessageType type)
{
    static const std::unordered_map<MessageType, Priority> kMap{
        {MessageType::M1, 0},
        {MessageType::M2, 1},
        {MessageType::M3, 2},
        {MessageType::M4, 3},
        {MessageType::M5, 4},
    };
    auto it = kMap.find(type);
    if (it == kMap.end())
        return std::nullopt;
    return std::optional<Priority>(it->second);
}

struct MessageFragment
{
    MessageType type;
    std::vector<uint8_t> data; // Fragment of the message data
};

struct Message
{
    MessageType type;
    std::vector<uint8_t> data; // Complete message data
};

struct UDPPacket
{
    std::vector<uint8_t> buffer; // Raw data of the UDP packet

    // Serialize multiple messages into a single UDP packet
    void packMessages(const std::vector<MessageFragment> &messages)
    {
        for (const auto &msg : messages)
        {
            auto priorityOpt = getPriority(msg.type);
            if (!priorityOpt)
                continue; // Skip if priority is not found

            // Reserve space for the header: 1 byte for type, 2 bytes for size
            buffer.push_back(static_cast<uint8_t>(msg.type));
            uint16_t size = msg.data.size();
            buffer.push_back(static_cast<uint8_t>(size >> 8));   // High byte of size
            buffer.push_back(static_cast<uint8_t>(size & 0xFF)); // Low byte of size

            // Append message data
            buffer.insert(buffer.end(), msg.data.begin(), msg.data.end());
        }
    }
};

bool send(const UDPPacket &packet)
{
    // Implementation of sending the packet via UDP
    // This function would use a UDP socket to send the packet.buffer data
    // For demonstration purposes, we are not implementing actual UDP send here
    return true;
}

UDPPacket receive()
{
    // Implementation of receiving a packet via UDP
    // This function would use a UDP socket to receive data into UDPPacket.buffer
    // For demonstration purposes, we are simulating a received packet
    UDPPacket packet;
    // ... (populate packet.buffer with received data)
    return packet;
}

int main()
{
    // Example usage:
    std::vector<MessageFragment> messages;
    // Populate `messages` with message fragments to send
    // ...

    UDPPacket packet;
    packet.packMessages(messages);

    if (send(packet))
    {
        std::cout << "Packet sent successfully!" << std::endl;
    }

    UDPPacket receivedPacket = receive();
    // Process `receivedPacket` to extract message fragments
    // ...

    return 0;
}
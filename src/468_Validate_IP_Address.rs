// 468_Validate_IP_Address
// https://leetcode.cn/problems/validate-ip-address/description/

/// 给定一个字符串 queryIP。如果是有效的 IPv4 地址，返回 "IPv4" ；
/// 如果是有效的 IPv6 地址，返回 "IPv6" ；如果不是上述类型的 IP 地址，返回 "Neither" 。

/// 有效的IPv4地址 是 “x1.x2.x3.x4” 形式的IP地址。 其中 0 <= xi <= 255 且 xi 不能包含 前导零。
/// 例如: “192.168.1.1” 、 “192.168.1.0” 为有效IPv4地址， “192.168.01.1” 为无效IPv4地址; “192.168.1.00” 、 “192.168@1.1” 为无效IPv4地址。

/// 一个有效的IPv6地址 是一个格式为“x1:x2:x3:x4:x5:x6:x7:x8” 的IP地址，其中:

/// 1 <= xi.length <= 4
/// xi 是一个 十六进制字符串 ，可以包含数字、小写英文字母( 'a' 到 'f' )和大写英文字母( 'A' 到 'F' )。
/// 在 xi 中允许前导零。
/// 例如 "2001:0db8:85a3:0000:0000:8a2e:0370:7334" 
/// 和 "2001:db8:85a3:0:0:8A2E:0370:7334" 是有效的 IPv6 地址，
/// 而 "2001:0db8:85a3::8A2E:037j:7334" 和 "02001:0db8:85a3:0000:0000:8a2e:0370:7334" 是无效的 IPv6 地址。


// IPv6（Internet Protocol version 6）是互联网协议的第六版，是为了解决IPv4地址耗尽问题而设计的。
// IPv6的主要改进包括更大的地址空间、更好的路由和寻址机制、更好的安全性等。

// 更大的地址空间：IPv6地址长度为128位，比IPv4的32位地址长度大很多。
// 这意味着IPv6可以提供大约3.4*10^38个唯一的IP地址，远远超过IPv4的约43亿个地址。这使得每个设备都可以有一个唯一的公网IP地址，从而简化了网络配置和管理。

// 更好的路由和寻址机制：IPv6的设计使得路由表可以更加紧凑，路由效率更高。此外，IPv6还支持更复杂的寻址机制，如多播和任播。

// 更好的安全性：IPv6在设计时就考虑到了安全问题，它支持 IPsec（Internet Protocol Security），可以提供端到端的加密和认证。

// 无状态地址自动配置（SLAAC）：IPv6还引入了无状态地址自动配置机制，使得设备可以自动获取IPv6地址，无需通过DHCP服务器。

// IPv6地址通常表示为8组由冒号分隔的16位十六进制数，例如：2001:0db8:85a3:0000:0000:8a2e:0370:7334。
// 为了简化，可以省略前导零，并且每个地址中的一系列零可以用双冒号替换一次，例如：2001:db8:85a3::8a2e:370:7334。

// 尽管IPv6有很多优点，但其全球部署仍在进行中，主要原因是需要在硬件、软件和网络设备上进行大量的升级和配置工作。
// 然而，随着IPv4地址的耗尽，IPv6的使用将会越来越普遍。


// 无状态地址自动配置（Stateless Address Autoconfiguration，SLAAC）是IPv6协议中的一项功能，它允许设备自动配置IPv6地址，无需通过DHCP服务器或手动配置。
// SLAAC的工作原理如下：
// 链路本地地址的生成：当一个设备启动并连接到网络时，它首先会为其网络接口生成一个链路本地地址。
// 这个地址的前64位是特定的前缀FE80::，后64位是基于设备的MAC地址生成的。这个地址主要用于设备在本地网络中的通信。
// 路由器发现：设备会发送一个ICMPv6路由器请求消息，询问网络中的路由器信息。如果网络中有路由器，它们会回应一个路由器通告消息，其中包含了网络的前缀信息。
// 全局地址的生成：设备会根据收到的网络前缀和自己的接口标识（通常是MAC地址）生成一个全局唯一的IPv6地址。这个地址可以用于Internet通信。
// 地址的唯一性检测：在使用新生成的地址之前，设备会通过发送一个邻居请求消息来检查网络中是否有其他设备已经使用了这个地址。如果没有设备回应，那么这个地址就可以使用。
// 地址的使用：设备现在可以使用新生成的地址进行通信了。
// SLAAC的优点是配置过程完全自动化，设备无需手动配置或运行DHCP客户端。
// 但是，它也有一些限制，例如无法自动配置DNS服务器地址，这需要通过其他机制（如DHCPv6或手动配置）来完成。


// 库函数
impl Solution {
    pub fn valid_ip_address(query_ip: String) -> String {
        use std::net::{Ipv4Addr,Ipv6Addr};
        use std::str::FromStr;
        if query_ip.contains('.') { //IPV4
            if let Result::Ok(_a) = Ipv4Addr::from_str(&query_ip) {
                "IPv4".to_string()
            } else {
                "Neither".to_string()
            }
        } else if query_ip.contains("::") { //IPV6省略写法::本题中认为是错误写法，所以先单独判断
            "Neither".to_string()
        } else if query_ip.contains(':') { //IPV6
            if let Result::Ok(_a) = Ipv6Addr::from_str(&query_ip) {
                "IPv6".to_string()
            } else {
                "Neither".to_string()
            }
        } else {
            "Neither".to_string()
        }
    }
}


impl Solution {
    fn valid_ipv4(ip: &str) -> bool {
        let parts: Vec<&str> = ip.split('.').collect();
        if parts.len() != 4 {
            return false;
        }
        for part in parts {
            if part.is_empty() || (part.len() > 1 && part.starts_with('0')) || part.len() > 3 {
                return false;
            }
            if let Ok(num) = part.parse::<u8>() {
                continue;
            } else {
                return false;
            }
        }
        true
    }
    
    fn valid_ipv6(ip: &str) -> bool {
        let parts: Vec<&str> = ip.split(':').collect();
        if parts.len() != 8 {
            return false;
        }
        for part in parts {
            if part.len() > 4 || part.is_empty() {
                return false;
            }
            if let Ok(_) = u16::from_str_radix(part, 16) {
                continue;
            } else {
                return false;
            }
        }
        true
    }
    
    pub fn valid_ip_address(ip: String) -> String {
        if Self::valid_ipv4(&ip) {
            "IPv4".to_string()
        } else if Self::valid_ipv6(&ip) {
            "IPv6".to_string()
        } else {
            "Neither".to_string()
        }
    }
}
    

pub fn valid_ip_address(ip: String) -> String {
    let is_ipv4 = ip.split('.').count() == 4
        && ip.split('.')
            .filter(|&x| {
                x.parse::<u8>().is_ok()
                    && x.parse::<u8>().unwrap().to_string() == *x
            })
            .count()
            == 4;
    let is_ipv6 = ip.split(':').count() == 8
        && ip.split(':')
            .filter(|&x| {
                !x.is_empty()
                    && x.len() <= 4
                    && i64::from_str_radix(x, 16).is_ok()
            })
            .count()
            == 8;
    if is_ipv4 {
        "IPv4".to_string()
    } else if is_ipv6 {
        "IPv6".to_string()
    } else {
        "Neither".to_string()
    }
}


pub fn valid_ip_address(ip: String) -> String {
    // 检查是否可能是IPv4地址，即是否有四个部分
    let is_ipv4 = ip.split('.').count() == 4
        && ip.split('.')
            // 对每一部分进行过滤，只保留那些可以转换为u8类型（即0到255的整数）的部分
            // 并且转换回字符串后与原字符串相同（这样可以排除有前导零的部分）
            .filter(|&x| {
                x.parse::<u8>().is_ok()
                    && x.parse::<u8>().unwrap().to_string() == *x
            })
            // 检查过滤后的部分是否仍然有四个，如果是，那么这是一个有效的IPv4地址
            .count()
            == 4;

    // 检查是否可能是IPv6地址，即是否有八个部分
    let is_ipv6 = ip.split(':').count() == 8
        && ip.split(':')
            // 对每一部分进行过滤，只保留那些非空且长度不超过4的部分
            // 并且可以解析为十六进制数的部分
            .filter(|&x| {
                !x.is_empty()
                    && x.len() <= 4
                    && i64::from_str_radix(x, 16).is_ok()
            })
            // 检查过滤后的部分是否仍然有八个，如果是，那么这是一个有效的IPv6地址
            .count()
            == 8;
            
    // 返回结果
    if is_ipv4 {
        "IPv4".to_string()
    } else if is_ipv6 {
        "IPv6".to_string()
    } else {
        "Neither".to_string()
    }
}

// 468_Validate_IP_Address
// https://leetcode.cn/problems/validate-ip-address/description/

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

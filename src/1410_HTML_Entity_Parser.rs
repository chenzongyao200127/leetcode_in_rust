// 1410_HTML_Entity_Parser
// https://leetcode.cn/problems/html-entity-parser/description/

impl Solution {
    pub fn entity_parser(text: String) -> String {
        let mut result = text;
        result = result.replace("&quot;", "\"");
        result = result.replace("&apos;", "'");
        result = result.replace("&gt;", ">");
        result = result.replace("&lt;", "<");
        result = result.replace("&frasl;", "/");
        result = result.replace("&amp;", "&");
        result
    }
}


// The `replace()` method in Rust is a string method used to create a new string 
// where all occurrences of a specified pattern in the original string are replaced with a 
// given replacement. This method is particularly useful for string manipulation tasks such as 
// replacing substrings, removing unwanted characters, or formatting.

// Here's a detailed explanation of how the `replace()` method works:

// 1. **Syntax**: 
//    ```rust
//    replace(pattern: P, replacement: &str) -> String
//    ```
//    - `pattern`: This is the substring or pattern you want to find in the string. 
//      The pattern can be a borrowed slice of type `&str` or any type that implements the `Pattern` trait, such as a character.
//    - `replacement`: This is the string that will replace each occurrence of the pattern in the original string.
//    - The method returns a new `String`.

// 2. **Functionality**: 
//    - The `replace()` method scans the original string for occurrences of the specified pattern.
//    - Each time the pattern is found, it is replaced with the replacement string.
//    - This process continues until all occurrences of the pattern have been replaced.
//    - The method returns a new string with all the replacements made.

// 3. **Case Sensitivity**: 
//    - The search for the pattern is case-sensitive. 
//      This means that the method will distinguish between uppercase and lowercase characters.

// 4. **Usage Example**:
//    ```rust
//    let original = "Hello, world!";
//    let replaced = original.replace("world", "Rust");
//    println!("{}", replaced); // Outputs: "Hello, Rust!"
//    ```

// 5. **Performance Considerations**:
//    - Since `replace()` returns a new `String`, 
//      it involves memory allocation for the new string. 
//      If you are replacing patterns in a loop or in a performance-critical part of your code, 
//      it's important to consider the performance implications.

// 6. **Limitations**:
//    - The `replace()` method is not suitable for complex pattern matching like regular expressions. 
//      If you need more complex pattern matching, consider using the `regex` crate.

// 7. **Common Use Cases**:
//    - Cleaning up strings by replacing unwanted characters or substrings.
//    - Formatting strings, such as replacing placeholders with actual values.
//    - Simple text processing tasks where regular expressions are not required.

// In summary, the `replace()` method in Rust is a straightforward and convenient way to replace 
// parts of a string with a different substring, and it's widely used for various string manipulation tasks.
Conclusion:

By using the clone method, you can create a deep copy of a String in Rust, allowing you to retain ownership of the original string (text2)
while also creating a new owned copy (text3). This allows both strings to be used independently, avoiding the move semantics issue
that would otherwise make text2 invalid after assignment.

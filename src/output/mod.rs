/*
    Build a buffer of sorts.

    output::write(str) will push 'str' to the buffer
    and then output the entire buffer to the console with a '\r' at
    the start (no windows support whatever lol)

    That way you can sort the buffer on the first few characters
    and make sure that everything started with 'Building' comes first
    'Done' second, 'Running' third, etc.
 */
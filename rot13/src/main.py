from string import ascii_lowercase, ascii_uppercase

def rot13_char(c):
    if c.isalpha():
        alphabet = ascii_uppercase if c.isupper() else ascii_lowercase
        return alphabet[(alphabet.find(c) + 13) % 26]
    else:
        return c

def rot13(message):
    return ''.join([rot13_char(c) for c in message])
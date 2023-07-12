import secrets
l = int(input('Enter length (will create both access and refresh token secrets): '))
print(secrets.token_hex(l))
print(secrets.token_hex(l))
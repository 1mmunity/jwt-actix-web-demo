--- @block insertUser
INSERT INTO users (username, email, password) VALUES (
  'John Smith',
  'johnsmith@gmail.com',
  crypt('fishcake', gen_salt('bf'))
) RETURNING *;

-- @block loginUser
SELECT * FROM users WHERE email = 'johnsmith@gmail.com' AND password = crypt('fishcake', password);
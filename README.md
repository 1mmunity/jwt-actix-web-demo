# JWT Rust actix-web User Auth Demo
This is a simple web api using actix-web for JWT auth.  
This API will run on `localhost:3001`

> **Note:** haven't tested on frontend, if you've encountered any CORS problem,
> please contact me (or open an issue)

## (Production) use the following command
To start the production server, simply run the following command  
This requires no setup but docker.
```sh
$ docker compose up
```

## (Development) use the following command  
Note: **You need to HAVE a Redis and PostgreSQL server running, and define their url in .env**
```sh
$ cargo run
```


# Technologies
- **Rust** (actix-web) for web api
- **PostgreSQL** for user storage
- **Redis** for refresh token storage

# Thunder Client
for **VSCode Thunder Client Extension**, you can use the json file provided in the **`extras`** folder

# Steps
0. (optional) Ping server **`/`**
1. Create/Login into an account **`/auth`**
   1. Store **`access_token`** in **`localStorage`**, this is stateless and is not stored anywhere in the backend. (lasts 5 minutes)
   2. From response header cookie, store the **`jwt`** cookie in your cookiejar. **(this is the refresh token, lasts 2 days, stored in redis)**
   3. The response also contains the user information.
2. Access protected route **`/me`**, this contains user information
   1. **Note: always append your access token in the `Authorization` header, as `Bearer {access_token}`**
   2. Once your access token has expired, it will send
   ```json
    {
      "success": false,
      "status": 401,
      "content": {
        "error_code": "EXPIRED_ACCESS_TOKEN",
        "message": "your access token has expired, please refresh at /auth/refreshAccessToken",
        "error_type": "AuthError"
      }
    }
   ```
3. Refresh the access token at **`GET /auth/refreshAccessToken`**, this will also cycle the refresh token and replace it in redis (each user can only have one refresh token).
   1. must inclue cookie header: jwt={refresh_token}
4. Cycle and repeat.
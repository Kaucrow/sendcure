# Basic OpenLDAP Server in Docker

A minimal OpenLDAP container setup using `osixia/openldap`.

## Quick start
0. Create your env file:
	 ```bash
	 cp .env.example .env
	 ```
	Then edit `.env` with your local secrets and router host.

1. Start the server:
	 ```bash
	 docker compose up -d
	 ```
2. Verify its running:
	 ```bash
	 docker ps
	 ```
3. Login to CLIs with:
    ```bash
	 docker attach (CLI name)
	 ```
4. Use to logout
     ```bash
        docker compose down -v
    ```


## User logins

Each user logs in (binds) with their full DN and password. The server is
available at `ldap://localhost:389` and the base DN is `dc=atlas,dc=com`.

User bind DNs and passwords:

- Andres Garcia: `uid=andres.garcia,ou=users,dc=atlas,dc=com` / `jojos`
- Juan Perez: `uid=juan,ou=users,dc=atlas,dc=com` / `jojos`
- Maria Lopez: `uid=maria,ou=users,dc=atlas,dc=com` / `jojos`
- Pedro Ruiz: `uid=pedro,ou=users,dc=atlas,dc=com` / `jojos`

Example login:

```bash
ldapwhoami -x -H ldap://localhost:389 -D "uid=juan,ou=users,dc=atlas,dc=com" -w jojos
```
the app should get the users id and password and then use an ldap library to made the search

## traefik
to see the traefik dashboard in your browser go to the url
```bash
dashboard.docker.localhost
```

## Traefik TLS (local self-signed)

- TLS options are defined in `traefik-dynamic.yml` under `tls.options.modern` and applied to routers with:
	- `traefik.http.routers.<name>.tls.options=modern@file`
- Certificates are generated locally by the `openssl` service and loaded by Traefik from `./certs`.
- Set `TRAEFIK_FRONTEND_HOST` in `.env` to the host you use in development (for example `atlas.com` in your local hosts file).
# tagparam Documentation

`tagparam` was created for CI/CD purposes where you might be inclined to
need to pick elements out of your git tag without having to introduce
any shell scripting to split and retrieve values.

Usage: tagparm delimiter index tag-string

Example: `tagparm - 1 dev-appX-uuid`

Results in the parameter: `dev`

Example `tagparm + 2 stage+appX+hash`

Results in the parameter: `appX`

## Docker

Examples:

`docker run -it hunterkirk/tagparam:v0.3.7 -h`

`docker run -it hunterkirk/tagparam:latest -h`

`docker run -it hunterkirk/tagparam:latest - 2 prod-api-8e42fgh`

<hr>

v0.3.7 Requires Rust v1.61.0

<hr>
Last Updated: 06-26-2022

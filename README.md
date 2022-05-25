# axum-maria-test

There are 2 routes with both POST and GET available.

`/login`:
- `GET`: Retrieves the HTML form to input data
- `POST`: Gets a vector of the email and password matches. If there’s a single one, the user can login.

`/register`
- `GET`: Retrieves the HTML form to input data
- `POST`: Gets a vector of the email matches. If there’s more than 0, then it means there is an account already with that email. Hence, the user can’t create an account. Otherwise, the account is registered in a MariaDB table

`/`
- `GET`: Just shows the main page
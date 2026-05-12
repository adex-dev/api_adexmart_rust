#Standard status code database
```bash

#[derive(Debug)]
pub enum UserStatus {
Inactive = 0,
Active = 1,
Suspended = 2,
Banned = 3,
Deleted = 4,
}
```

#Standard Status code Response
sample

```bash
#[derive(Debug)]
pub enum StatusCodeResponse {
    access_auth/login = 0,
    Created = 1,
    validasi = 2,
    conflict = 3,
    not_found = 4,
    server_error = 5,
    bad_request=6,
    access_normal=7,
    Databasedown=8,
}
```
#Example Response
- status_label = title pada alert
- status = code validasi response yang diterapkan pada frontend


```bash
{
"id": 10,d
"name": "Akmad",
"status": 3,
"status_label": "banned",
"message": "User diblokir, silakan hubungi admin."
}
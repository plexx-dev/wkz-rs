# wkz-rs [![Docker Image CI](https://github.com/plexx-dev/wkz-rs/actions/workflows/docker-image.yml/badge.svg)](https://github.com/plexx-dev/wkz-rs/actions/workflows/docker-image.yml)
simple tool periodically fetch your german Wunschkennzeichen

## config
create a file called ``config.cfg`` and use the following file structure (CASE SENSITIVE)

```
{
    "email": {
        "sender": "example@example.com",
        "subject": "Dein Wunschkennzeichen ist frei: ",
        "smtp_username": "example@example.com",
        "smtp_password": "password",
        "smtp_server": "mail.example.com"
    },
    "wkzs": [
        {
            "pattern": "F GG ?",
            "city": 623,
            "receiver": "example@example.com",
            "email_alert": true
        },
        {
            "pattern": "HH GG ??",
            "city": 743,
            "receiver": "example@example.com",
            "email_alert": false
        }
    ]
}
```

to find out what your city code is follow these steps:
1. go to the website https://wunschkennzeichen.zulassung.de/wunschkennzeichen
2. open the networks tab
3. enter your desired numberplate and hit enter
4. check the only POST request that should be there
5. go to the request body
5. copy the number under ``registrationOfficeServiceId``
6. have fun :D

# Rust HP Scan to

## Example Docker Compose file
```yml
version: '3.3'
services:
    hp-scan-to:
        restart: unless-stopped
        # this is needed for direct communication with the printer
        network_mode: host
        image: ghcr.io/daullmer/rust-hp-scan-to
        environment:
            - MAIL_FROM=<<sender mail>>
            - MAIL_TO=<<recipient mail>>
            - PRINTER_URL=http://<<printer_url>>
            - SCAN_NAME='to mail'
            - SENDGRID_API_KEY=<<api_key>>
```

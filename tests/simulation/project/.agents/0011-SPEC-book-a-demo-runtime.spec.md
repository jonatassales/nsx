# Book a demo as a microruntime

## Components

- `services/signup-api` `:8080`
- `services/notify-api`
- `platform/nats`
- `platform/redis`
- `apps/signup`

## Acceptance criteria

- [ ] The sign-up API records a demo request from the sign-up page.
- [ ] The notify API tells the team through the broker.
- [ ] The cache keeps the sign-up page's state between requests.

# Welcome to my newsletter API

After the completion of this API I will be integrating the project into my portfolio site which will be located at bornedj.me.
The API and portfolio will allow me to share several of the projects I've been working on in the past few years as well as reach out to whomever is interested through a recurring newsletter.

## This API was built with the guidance of Luca Palmieri's [Zero to Production In Rust](https://www.zero2prod.com/)

## Technologies

1. Testing
   - Fake for faking data for unit and integration tests.
   - HTTP client mocking with wiremock and reqwest.
2. Telemetry
   - logging done with actix-web middleware crate
   - structured logging done with tracing and tracing-log crates
3. CI/CD
   - This project is hosted within a docker container on [digital oceans app platform](https://newsletter-styiq.ondigitalocean.app/health_check)
   - CI done with github actions/workflows.
4. backend
   - API built with the actix-web framework/crate.
   - Asynchronous multithreaded programming through tokio.
   - Postgres RDBMS instance within a docker container.
   - adding a redis in memory database for caching cookies.
   - Compile-time checked queries without a DSL done with sqlx.
   - [Postmark](https://postmarkapp.com/) for an email delivery service.

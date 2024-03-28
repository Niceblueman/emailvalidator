<div align="center">
  <h1>Email Verification API</h1>
</div>

## Purpose

The Email Verification API serves to check and verify email addresses for developers, providing comprehensive information on email syntax validity, SMTP verification, domain restrictions, disposable email detection, and more.

## Features

- **Email Syntax Validation**: Ensure emails adhere to correct syntax standards.
- **MX-Records & SMTP Verification**: Verify email servers' existence and responsiveness.
- **Catch-all Detection**: Identify if an email server accepts emails for any address.
- **Free & Disposable Email Filtering**: Distinguish between free/disposable emails and genuine domains.
- **Deliverability Score**: Assess the likelihood of emails being delivered successfully.
- **Real-time SMTP Testing**: Dynamically assess email server configurations.
- **Role Email Address Filtering**: Filter out emails typically associated with functions rather than individuals.
- **Quality Score Tracking**: Monitor usage and receive feedback on the quality of email addresses.
- **Interactive Documentation**: Explore API features and functionalities interactively.
- **Extended Usage Statistics**: Gain insights into API usage patterns and trends.

## Security

The API ensures security through 256-bit HTTPS encryption, safeguarding data transmissions against unauthorized access.

## Usage

Developers can seamlessly integrate the API into their systems using a straightforward URL structure, receiving responses in lightweight JSON format for easy consumption and processing.

## Support

The API provider offers world-class technical support, ensuring users receive timely assistance and guidance to address their needs effectively.

## Future Plans

Future plans include introducing additional APIs such as IP score and email template validator. Additionally, SDKs for Python, JavaScript, Rust, and PHP are in the pipeline to enhance integration capabilities and developer experience.

## Technologies Used

- **Rust**: Backend development
- **ReactJS**: Frontend development
- **styled-components**: Styling the frontend components

# run it locally:
```bash
git clone <repo:main>
```
```bash
cd back && cargo run
```
__note__: the front is built and copied into template folder to be user in rust http server [rocket](https://api.rocket.rs/v0.5/rocket/) as [handle bars template](https://handlebarsjs.com/)

## Documentation

Interactive documentation is readily available for users to explore the API's features and functionalities, facilitating seamless integration and usage.
# API ROUTES (benerated from rocket_logs):

📬 Routes:
   >> (mainpage) GET /
   >> (check_handler) POST /check application/json
   >> (login) POST /login application/json
   >> (getplans) POST /plans
   >> (recover) POST /recover application/json
   >> (profile) GET /profile
   >> (manage_plans) POST /myplans application/json
   >> (payment_link) POST /payment application/json
   >> (register) POST /register application/json
   >> (pay_back) POST /pay_back application/json
   >> (pay_back_get) GET /pay_back
   >> (demo_check) POST /demoCheck application/json
   >> (check_bulk) POST /check_bulk application/json
   >> GET /openapi.json
   >> (FileServer: ./templates/feed) GET /rss/<path..> [10]
   >> (FileServer: ./templates/icons) GET /icons/<path..> [10]
   >> (FileServer: ./templates/static) GET /static/<path..> [10]
   >> (FileServer: ./templates/assets) GET /assets/<path..> [10]
   >> GET /rapidoc/
   >> GET /rapidoc/index.html
   >> GET /rapidoc/rapidoc-min.js
   >> GET /rapidoc/oauth-receiver.html
   >> GET /swagger-ui/
   >> (FileServer: ./templates/Background) GET /Background/<path..> [10]
   >> GET /swagger-ui/index.css
   >> GET /swagger-ui/index.html
   >> GET /swagger-ui/swagger-ui.css
   >> GET /swagger-ui/oauth2-redirect.html
   >> GET /swagger-ui/swagger-ui-bundle.js
   >> GET /swagger-ui/swagger-ui-config.json
   >> GET /swagger-ui/swagger-initializer.js
   >> GET /swagger-ui/swagger-ui-standalone-preset.js
🥅 Catchers:
   >> (bad_request) 400
   >> (unauthorized) 401
   >> (not_found) 404
   >> (internal_error) 500

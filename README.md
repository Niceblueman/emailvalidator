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
# emailvalidator

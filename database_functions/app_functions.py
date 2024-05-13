def send_email(server_name, server_port, from_email, to_email, send_mode, encryption, auth_required, username, password, subject, body):
    import smtplib
    from email.mime.multipart import MIMEMultipart
    from email.mime.text import MIMEText
    import ssl
    import socket

    try:
        if send_mode == "SMTP":
            # Set up the SMTP server.
            if encryption == "SSL/TLS":
                smtp = smtplib.SMTP_SSL(server_name, server_port, timeout=10)
            elif encryption == "STARTTLS":
                smtp = smtplib.SMTP(server_name, server_port, timeout=10)
                smtp.starttls()
            else:  # No encryption
                smtp = smtplib.SMTP(server_name, server_port, timeout=10)


            # Authenticate if needed.
            if auth_required:
                try:  # Trying to login and catching specific SMTPNotSupportedError
                    smtp.login(username, password)
                except smtplib.SMTPNotSupportedError:
                    return 'SMTP AUTH extension not supported by server.'

            # Create a message.
            msg = MIMEMultipart()
            msg['From'] = from_email
            msg['To'] = to_email
            msg['Subject'] = subject
            msg.attach(MIMEText(body, 'plain'))

            # Send the message.
            smtp.send_message(msg)
            smtp.quit()
            return 'Email sent successfully.'

        elif send_mode == "Sendmail":
            pass
    except ssl.SSLError:
        return 'SSL Wrong Version Number. Try another ssl type?'
    except smtplib.SMTPAuthenticationError:
        return 'Authentication Error: Invalid username or password.'
    except smtplib.SMTPRecipientsRefused:
        return 'Recipients Refused: Email address is not accepted by the server.'
    except smtplib.SMTPSenderRefused:
        return 'Sender Refused: Sender address is not accepted by the server.'
    except smtplib.SMTPDataError:
        return 'Unexpected server response: Possibly the message data was rejected by the server.'
    except socket.gaierror:
        return 'Server Not Found: Please check your server settings.'
    except ConnectionRefusedError:
        return 'Connection Refused: The server refused the connection.'
    except TimeoutError:
        return 'Timeout Error: The connection to the server timed out.'
    except smtplib.SMTPException as e:
        return f'Failed to send email: {str(e)}'



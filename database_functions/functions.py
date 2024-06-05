import random
import string
import mysql.connector
from mysql.connector import errorcode
import mysql.connector.pooling
import sys
import os
import requests
import datetime
import time
import appdirs
import base64
import subprocess
import psycopg2
from psycopg2.extras import RealDictCursor
from requests.exceptions import RequestException

# # Get the application root directory from the environment variable
# app_root = os.environ.get('APP_ROOT')
sys.path.append('/nettailor/')

def get_web_key(cnx):
    cursor = cnx.cursor()
    query = "SELECT APIKey FROM APIKeys WHERE UserID = 1"
    cursor.execute(query)
    result = cursor.fetchone()
    cursor.close()

    if result:
        return result[0]
    else:
        return None



def add_user(cnx, user_values):
    cursor = cnx.cursor()

    add_user = ("INSERT INTO Users "
                "(Fullname, Username, Email, Hashed_PW, IsAdmin) "
                "VALUES (%s, %s, %s, %s, 0)")

    cursor.execute(add_user, user_values)

    user_id = cursor.lastrowid

    add_user_settings = ("INSERT INTO UserSettings "
                         "(UserID, Theme) "
                         "VALUES (%s, %s)")

    cursor.execute(add_user_settings, (user_id, 'nordic'))

    add_user_stats = ("INSERT INTO UserStats "
                      "(UserID) "
                      "VALUES (%s)")

    cursor.execute(add_user_stats, (user_id,))

    cnx.commit()

    cursor.close()
    # cnx.close()

def add_azure_user(cnx, user_info):
    cursor = cnx.cursor()
    
    try:
        # Assuming 'name' and 'email' are provided in user_info, and no password is needed
        fullname = user_info.get('name', '')
        username = user_info.get('email').split('@')[0]  # Simplistic username generation from email
        email = user_info.get('email')
        hashed_pw = 'external'  # Placeholder or use NULL if your DB schema allows

        add_user_query = ("INSERT INTO Users "
                          "(Fullname, Username, Email, Hashed_PW, IsAdmin) "
                          "VALUES (%s, %s, %s, %s, 0)")
        cursor.execute(add_user_query, (fullname, username, email, hashed_pw))

        user_id = cursor.lastrowid

        # Add default settings for the new user
        add_user_settings_query = ("INSERT INTO UserSettings "
                                   "(UserID, Theme) "
                                   "VALUES (%s, 'nordic')")
        cursor.execute(add_user_settings_query, (user_id,))

        # Add default stats for the new user
        add_user_stats_query = ("INSERT INTO UserStats "
                                "(UserID) "
                                "VALUES (%s)")
        cursor.execute(add_user_stats_query, (user_id,))

        cnx.commit()

        # Return some form of user identifier or object
        return user_id
    except Exception as e:
        cnx.rollback()
        raise e
    finally:
        cursor.close()


def add_external_auth(cnx, external_auth_values):
    cursor = cnx.cursor()

    add_auth_query = ("INSERT INTO ExternalAuth "
                        "(Provider, ClientID, TenantID, RedirectURI, Secret) "
                        "VALUES (%s, %s, %s, %s, %s)")

    # Use the attributes from the Pydantic model
    cursor.execute(add_auth_query, (external_auth_values.provider, external_auth_values.client_id, 
                                    external_auth_values.tenant_id, external_auth_values.redirect_uri, 
                                    external_auth_values.secret))

    cnx.commit()
    cursor.close()

def get_all_external_auths(cnx):
    cursor = cnx.cursor()
    try:
        query = "SELECT Provider, ClientID, TenantID, RedirectURI, Secret FROM ExternalAuth"
        cursor.execute(query)
        results = cursor.fetchall()
        return results
    finally:
        cursor.close()

def get_azure_auth(cnx):
    cursor = cnx.cursor()
    try:
        query = "SELECT ClientID, TenantID, RedirectURI, Secret FROM ExternalAuth WHERE provider = 'Azure'"
        cursor.execute(query)
        result = cursor.fetchone()
        if result:
            return {
                "client_id": result[0],
                "tenant_id": result[1],
                "redirect_uri": result[2],
                "client_secret": result[3]
            }
        else:
            # Return None or an empty dict to indicate no settings were found
            return None
    except Exception as e:
        logging.error("Error fetching Azure auth settings from the database", exc_info=True)
        # Optionally, you can still log the error for diagnostics without raising an exception
        return None
    finally:
        cursor.close()



def add_admin_user(cnx, user_values):
    cursor = cnx.cursor()

    add_user = ("INSERT INTO Users "
                "(Fullname, Username, Email, Hashed_PW, IsAdmin) "
                "VALUES (%s, %s, %s, %s, %s, 1)")

    cursor.execute(add_user, user_values)

    user_id = cursor.lastrowid

    add_user_settings = ("INSERT INTO UserSettings "
                         "(UserID, Theme) "
                         "VALUES (%s, %s)")

    cursor.execute(add_user_settings, (user_id, 'nordic'))

    add_user_stats = ("INSERT INTO UserStats "
                      "(UserID) "
                      "VALUES (%s)")

    cursor.execute(add_user_stats, (user_id,))

    cnx.commit()

    cursor.close()


def exchange_code_for_token(cnx, code):
    try:
        azure_config = get_azure_config(cnx)
        token_url = f"https://login.microsoftonline.com/{azure_config['TenantID']}/oauth2/v2.0/token"
        data = {
            "grant_type": "authorization_code",
            "client_id": azure_config['ClientID'],
            "client_secret": azure_config['Secret'],
            "code": code,
            "redirect_uri": azure_config['RedirectURI'],
            "scope": "openid email profile"
        }
        
        response = requests.post(token_url, data=data)
        logging.debug(f"Request sent to Azure Token URL: {token_url} with data {data}")
        logging.debug(f"Response from Azure: {response.text}")

        if response.status_code == 200:
            return response.json()
        else:
            error_message = response.json().get('error_description', 'No error description provided')
            logging.error(f"Failed to exchange code for token: {error_message}")
            raise Exception(f"Failed to exchange code for token: {error_message}")
    except Exception as e:
        logging.error(f"Error during token exchange: {str(e)}", exc_info=True)
        raise Exception(f"Error during token exchange: {str(e)}")



def remove_user(cnx, user_name):
    pass


def check_self_service(cnx):
    cursor = cnx.cursor()
    query = "SELECT SelfServiceUser FROM AppSettings"
    cursor.execute(query)
    result = cursor.fetchone()
    cursor.close()

    if result and result[0] == 1:
        return True
    elif result and result[0] == 0:
        return False
    else:
        return None



def get_user_id(cnx, username):
    cursor = cnx.cursor()
    query = "SELECT UserID FROM Users WHERE Username = %s"
    cursor.execute(query, (username,))
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result:
        return result[0]
    else:
        return 1


def get_user_details(cnx, username):
    cursor = cnx.cursor()
    query = "SELECT * FROM Users WHERE Username = %s"
    cursor.execute(query, (username,))
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result:
        return {
            'UserID': result[0],
            'Fullname': result[1],
            'Username': result[2],
            'Email': result[3],
            'Hashed_PW': result[4]
        }
    else:
        return None

def get_user_details_email(cnx, email):
    cursor = cnx.cursor()
    query = "SELECT * FROM Users WHERE Email = %s"
    cursor.execute(query, (email,))
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result:
        return {
            'UserID': result[0],
            'Fullname': result[1],
            'Username': result[2],
            'Email': result[3],
            'Hashed_PW': result[4]
        }
    else:
        return None

def get_user_details_id(cnx, user_id):
    cursor = cnx.cursor()
    query = "SELECT * FROM Users WHERE UserID = %s"
    cursor.execute(query, (user_id,))
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result:
        return {
            'UserID': result[0],
            'Fullname': result[1],
            'Username': result[2],
            'Email': result[3],
            'Hashed_PW': result[4]
        }
    else:
        return None

def get_encryption_key(cnx):
    cursor = cnx.cursor()
    query = ("SELECT EncryptionKey FROM AppSettings WHERE AppSettingsID = 1")
    cursor.execute(query)
    result = cursor.fetchone()

    if not result:
        cursor.close()
        # cnx.close()
        return None

    # Convert the result to a dictionary.
    result_dict = dict(zip([column[0] for column in cursor.description], result))

    cursor.close()
    # cnx.close()

    # Convert the bytearray to a base64 encoded string before returning.
    return base64.b64encode(result_dict['EncryptionKey']).decode()


def get_email_settings(cnx):
    cursor = cnx.cursor()

    query = "SELECT * FROM EmailSettings"
    cursor.execute(query)

    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result:
        keys = ["EmailSettingsID", "Server_Name", "Server_Port", "From_Email", "Send_Mode", "Encryption",
                "Auth_Required", "Username", "Password"]
        return dict(zip(keys, result))
    else:
        return None


def check_usernames(cnx, username):
    cursor = cnx.cursor()
    query = ("SELECT COUNT(*) FROM Users WHERE Username = %s")
    cursor.execute(query, (username,))
    count = cursor.fetchone()[0]
    cursor.close()
    # cnx.close()
    return count > 0


def get_theme(cnx, user_id):
    cursor = None
    try:
        cursor = cnx.cursor()

        # Get the EpisodeID from the Episodes table
        query = "SELECT Theme FROM UserSettings WHERE UserID = %s"
        cursor.execute(query, (user_id,))
        theme = cursor.fetchone()[0]

        return theme

    finally:
        if cursor:
            cursor.close()
            # cnx.close()


def set_theme(cnx, user_id, theme):
    cursor = None
    try:
        cursor = cnx.cursor()

        # Update the UserSettings table with the new theme value
        query = "UPDATE UserSettings SET Theme = %s WHERE UserID = %s"
        cursor.execute(query, (theme, user_id))
        cnx.commit()

    finally:
        if cursor:
            cursor.close()
            # cnx.close()


def get_user_info(database_type, cnx):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    query = (f"SELECT Users.UserID, Users.Fullname, Users.Username, "
             f"Users.Email, Users.IsAdmin "
             f"FROM Users ")

    cursor.execute(query)
    rows = cursor.fetchall()

    cursor.close()
    # cnx.close()

    if not rows:
        return None

    return rows


def get_api_info(database_type, cnx, user_id):
    # Check if the user is an admin
    is_admin_query = "SELECT IsAdmin FROM Users WHERE UserID = %s"
    cursor = cnx.cursor()
    cursor.execute(is_admin_query, (user_id,))
    is_admin_result = cursor.fetchone()
    cursor.close()

    # Adjusting access based on the result type
    if isinstance(is_admin_result, dict):  # Dictionary style
        is_admin = is_admin_result.get('IsAdmin', 0)
    elif isinstance(is_admin_result, tuple):  # Tuple style (fallback)
        # Assuming 'IsAdmin' is the first column in the SELECT statement
        is_admin = is_admin_result[0] if is_admin_result else 0
    else:
        is_admin = 0


    # Adjust the query based on whether the user is an admin
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    query = (f"SELECT APIKeys.APIKeyID, APIKeys.UserID, Users.Username, "
             f"RIGHT(APIKeys.APIKey, 4) as LastFourDigits, "
             f"APIKeys.Created "
             f"FROM APIKeys "
             f"JOIN Users ON APIKeys.UserID = Users.UserID ")

    # Append condition to query if the user is not an admin
    if not is_admin:
        query += f"WHERE APIKeys.UserID = {user_id} "

    cursor.execute(query)
    rows = cursor.fetchall()
    cursor.close()
    # cnx.close()

    if not rows:
        return []

    return rows



def create_api_key(cnx, user_id):
    import secrets
    import string
    alphabet = string.ascii_letters + string.digits
    api_key = ''.join(secrets.choice(alphabet) for _ in range(64))

    cursor = cnx.cursor()
    query = "INSERT INTO APIKeys (UserID, APIKey) VALUES (%s, %s)"
    cursor.execute(query, (user_id, api_key))
    cnx.commit()
    cursor.close()
    # cnx.close()

    return api_key


def delete_api(cnx, api_id):
    cursor = cnx.cursor()
    query = "DELETE FROM APIKeys WHERE APIKeyID = %s"
    cursor.execute(query, (api_id,))
    cnx.commit()
    cursor.close()
    # cnx.close()


def set_username(cnx, user_id, new_username):
    cursor = cnx.cursor()
    query = "UPDATE Users SET Username = %s WHERE UserID = %s"
    cursor.execute(query, (new_username, user_id))
    cnx.commit()
    cursor.close()
    # cnx.close()


def set_password(cnx, user_id, hash_pw):
    cursor = cnx.cursor()
    update_query = "UPDATE Users SET Hashed_PW=%s WHERE UserID=%s"
    cursor.execute(update_query, (hash_pw, user_id))
    cnx.commit()
    cursor.close()



def set_email(cnx, user_id, new_email):
    cursor = cnx.cursor()
    query = "UPDATE Users SET Email = %s WHERE UserID = %s"
    cursor.execute(query, (new_email, user_id))
    cnx.commit()
    cursor.close()
    # cnx.close()


def set_fullname(cnx, user_id, new_name):
    cursor = cnx.cursor()
    query = "UPDATE Users SET Fullname = %s WHERE UserID = %s"
    cursor.execute(query, (new_name, user_id))
    cnx.commit()
    cursor.close()
    # cnx.close()


def set_isadmin(cnx, user_id, isadmin):
    cursor = cnx.cursor()

    # Convert boolean isadmin value to integer (0 or 1)
    isadmin_int = int(isadmin)

    query = f"UPDATE Users SET IsAdmin = {isadmin_int} WHERE UserID = {user_id}"

    cursor.execute(query)
    cnx.commit()

    cursor.close()
    # cnx.close()


def delete_user(cnx, user_id):
    cursor = cnx.cursor()


    # Delete user from UserSettings table
    try:
        query = "DELETE FROM UserSettings WHERE UserID = %s"
        cursor.execute(query, (user_id,))
    except:
        pass

    # Delete user from UserStats table
    try:
        query = "DELETE FROM UserStats WHERE UserID = %s"
        cursor.execute(query, (user_id,))
    except:
        pass

    # Delete user from Users table
    query = "DELETE FROM Users WHERE UserID = %s"
    cursor.execute(query, (user_id,))
    cnx.commit()

    cursor.close()
    # cnx.close()


def user_admin_check(cnx, user_id):
    cursor = cnx.cursor()
    query = f"SELECT IsAdmin FROM Users WHERE UserID = '{user_id}'"
    cursor.execute(query)
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result is None:
        return False

    return bool(result[0])


def final_admin(cnx, user_id):
    cursor = cnx.cursor()

    # Check if user being deleted is the final admin user
    query = "SELECT COUNT(*) FROM Users WHERE IsAdmin = 1"
    cursor.execute(query)
    admin_count = cursor.fetchone()[0]

    if admin_count == 1:
        query = "SELECT IsAdmin FROM Users WHERE UserID = %s"
        cursor.execute(query, (user_id,))
        is_admin = cursor.fetchone()[0]
        if is_admin == 1:
            return True

    cursor.close()
    # cnx.close()

    return False


def download_status(cnx):
    cursor = cnx.cursor()
    query = "SELECT DownloadEnabled FROM AppSettings"
    cursor.execute(query)
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result and result[0] == 1:
        return True
    else:
        return False


def guest_status(cnx):
    cursor = cnx.cursor()
    query = "SELECT Email FROM Users WHERE Email = 'active'"
    cursor.execute(query)
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result:
        return True
    else:
        return False


def enable_disable_guest(cnx):
    cursor = cnx.cursor()
    query = "UPDATE Users SET Email = CASE WHEN Email = 'inactive' THEN 'active' ELSE 'inactive' END WHERE Username = 'guest'"
    cursor.execute(query)
    cnx.commit()
    cursor.close()
    # cnx.close()


def enable_disable_downloads(cnx):
    cursor = cnx.cursor()
    query = "UPDATE AppSettings SET DownloadEnabled = CASE WHEN DownloadEnabled = 1 THEN 0 ELSE 1 END"
    cursor.execute(query)
    cnx.commit()
    cursor.close()
    # cnx.close()


def self_service_status(cnx):
    cursor = cnx.cursor()
    query = "SELECT SelfServiceUser FROM AppSettings WHERE SelfServiceUser = 1"
    cursor.execute(query)
    result = cursor.fetchone()
    cursor.close()
    # cnx.close()

    if result:
        return True
    else:
        return False

def verify_api_key(cnx, passed_key):
    cursor = cnx.cursor()
    query = "SELECT * FROM APIKeys WHERE APIKey = %s"
    cursor.execute(query, (passed_key,))
    result = cursor.fetchone()
    print(f"Result: {result}")
    cursor.close()
    return True if result else False


def get_api_key(cnx, username):
    try:
        with cnx.cursor() as cursor:
            # Get the UserID
            query = "SELECT UserID FROM Users WHERE username = %s"
            cursor.execute(query, (username,))
            result = cursor.fetchone()

            # Check if a result is returned. If not, return None
            if result is None:
                print("No user found with the provided username.")
                return None

            user_id = result[0]

            # Get the API Key using the fetched UserID, and limit the results to 1
            query = "SELECT APIKey FROM APIKeys WHERE UserID = %s LIMIT 1"
            cursor.execute(query, (user_id,))
            result = cursor.fetchone()

            # Check and return the API key or create a new one if not found
            if result:
                print(f"Result: {result}")
                return result[0]  # Adjust the index if the API key is in a different column
            else:
                print("No API key found for the provided user. Creating a new one...")
                return create_api_key(cnx, user_id)

    except Exception as e:
        print(f"An error occurred: {str(e)}")
        return f"An error occurred: {str(e)}"
    
def get_api_key_session(cnx, user_id):
    try:
        with cnx.cursor() as cursor:
            # Get the API Key using the fetched UserID, and limit the results to 1
            query = "SELECT APIKey FROM APIKeys WHERE UserID = %s LIMIT 1"
            cursor.execute(query, (user_id,))
            result = cursor.fetchone()

            # Check and return the API key or create a new one if not found
            if result:
                print(f"Result: {result}")
                return result[0]  # Adjust the index if the API key is in a different column
            else:
                print("No API key found for the provided user. Creating a new one...")
                return create_api_key(cnx, user_id)

    except Exception as e:
        print(f"An error occurred: {str(e)}")
        return f"An error occurred: {str(e)}"



def get_api_user(cnx, api_key):
    try:
        with cnx.cursor() as cursor:
            # Get the API Key using the fetched UserID, and limit the results to 1
            query = "SELECT UserID FROM APIKeys WHERE APIKey = %s LIMIT 1"
            cursor.execute(query, (api_key,))
            result = cursor.fetchone()

            # Check and return the API key or create a new one if not found
            if result:
                print(f"Result: {result}")
                return result[0]  # Adjust the index if the API key is in a different column
            else:
                print(f"ApiKey Not Found")
                return "ApiKey Not Found"

    except Exception as e:
        print(f"An error occurred: {str(e)}")
        return f"An error occurred: {str(e)}"


def id_from_api_key(cnx, passed_key):
    cursor = cnx.cursor()
    query = "SELECT UserID FROM APIKeys WHERE APIKey = %s"
    cursor.execute(query, (passed_key,))
    result = cursor.fetchone()
    print(f"Result: {result}")
    cursor.close()
    return result[0] if result else None


def check_api_permission(cnx, passed_key):
    import tempfile
    # Create a temporary file to store the content. This is because the mysql command reads from a file.
    with tempfile.NamedTemporaryFile(mode='w+', delete=True) as tempf:
        tempf.write(server_restore_data)
        tempf.flush()
        cmd = [
            "mysql",
            "-h", 'db',
            "-P", '3306',
            "-u", "root",
            "-p" + database_pass,
            "pypods_database"
        ]

        # Use the file's content as input for the mysql command
        with open(tempf.name, 'r') as file:
            process = subprocess.Popen(cmd, stdin=file, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            stdout, stderr = process.communicate()

            if process.returncode != 0:
                raise Exception(f"Restoration failed with error: {stderr.decode()}")

    return "Restoration completed successfully!"


def get_stats(cnx, user_id):
    cursor = cnx.cursor()

    # Query to get UserCreated and ConfigsCreated for the specific user
    user_query = ("SELECT UserCreated, ConfigsCreated "
                  "FROM UserStats "
                  "WHERE UserID = %s")
    cursor.execute(user_query, (user_id,))
    user_results = cursor.fetchall()
    user_result = user_results[0] if user_results else None

    # Query to get the total number of configurations
    total_configs_query = "SELECT COUNT(*) FROM Configurations"
    cursor.execute(total_configs_query)
    total_configs_result = cursor.fetchone()

    if user_result and total_configs_result:
        stats = {
            "UserCreated": user_result[0],
            "ConfigsCreated": user_result[1],
            "TotalConfigsCreated": total_configs_result[0]
        }
    else:
        stats = None

    cursor.close()
    # cnx.close()

    return stats


def increment_config_count(cnx, user_id):
    cursor = cnx.cursor()
    query = """
    UPDATE UserStats
    SET ConfigsCreated = ConfigsCreated + 1
    WHERE UserID = %s
    """
    try:
        cursor.execute(query, (user_id,))
        cnx.commit()
    except Exception as e:
        cnx.rollback()
        raise e
    finally:
        cursor.close()

def get_session_file_path():
    app_name = 'pinepods'
    data_dir = appdirs.user_data_dir(app_name)
    os.makedirs(data_dir, exist_ok=True)
    session_file_path = os.path.join(data_dir, "session.txt")
    return session_file_path


def save_session_to_file(session_id):
    session_file_path = get_session_file_path()
    with open(session_file_path, "w") as file:
        file.write(session_id)


def get_saved_session_from_file():
    app_name = 'pinepods'
    session_file_path = get_session_file_path()
    try:
        with open(session_file_path, "r") as file:
            session_id = file.read()
            return session_id
    except FileNotFoundError:
        return None


def check_saved_session(cnx, session_value):
    cursor = cnx.cursor()

    # Get the session with the matching value and expiration time
    cursor.execute("""
    SELECT UserID, expire FROM Sessions WHERE value = %s;
    """, (session_value,))

    result = cursor.fetchone()

    if result:
        user_id, session_expire = result
        current_time = datetime.datetime.now()
        if current_time < session_expire:
            return user_id

    return False
    cursor.close()
    # cnx.close()


def check_saved_web_session(cnx, session_value):
    cursor = cnx.cursor()

    # Get the session with the matching value and expiration time
    cursor.execute("""
    SELECT UserID, expire FROM Sessions WHERE value = %s;
    """, (session_value,))

    result = cursor.fetchone()

    if result:
        user_id, session_expire = result
        current_time = datetime.datetime.now()
        if current_time < session_expire:
            return user_id

    return False
    cursor.close()
    # cnx.close()


def create_session_for_user(cnx, user_id):
    import uuid
    import datetime
    cursor = cnx.cursor()
    session_id = str(uuid.uuid4())  # Generate a unique session identifier
    expiration_time = datetime.datetime.utcnow() + datetime.timedelta(days=1)  # 1 day expiration

    # Insert the session into the Sessions table
    insert_session_query = """
        INSERT INTO Sessions (UserID, value, expire)
        VALUES (%s, %s, %s)
    """
    cursor.execute(insert_session_query, (user_id, session_id, expiration_time))
    cnx.commit()
    cursor.close()

    return session_id


def create_web_session(cnx, user_id, session_value):
    # Calculate the expiration date 30 days in the future
    expire_date = datetime.datetime.now() + datetime.timedelta(days=30)

    # Insert the new session into the Sessions table
    cursor = cnx.cursor()
    cursor.execute("""
    INSERT INTO Sessions (UserID, value, expire) VALUES (%s, %s, %s);
    """, (user_id, session_value, expire_date))

    cnx.commit()
    cursor.close()
    # cnx.close()


def clean_expired_sessions(cnx):
    current_time = datetime.datetime.now()
    cursor = cnx.cursor()

    cursor.execute("""
    DELETE FROM Sessions WHERE expire < %s;
    """, (current_time,))

    cnx.commit()
    cursor.close()
    # cnx.close()


def user_exists(cnx, username):
    cursor = cnx.cursor()
    query = "SELECT COUNT(*) FROM Users WHERE Username = %s"
    cursor.execute(query, (username,))
    count = cursor.fetchone()[0]
    cursor.close()
    # cnx.close()
    return count > 0


def reset_password_create_code(cnx, user_email):
    reset_code = ''.join(random.choices(string.ascii_uppercase + string.digits, k=6))
    cursor = cnx.cursor()

    # Check if a user with this email exists
    check_query = """
        SELECT UserID
        FROM Users
        WHERE Email = %s
    """
    cursor.execute(check_query, (user_email,))
    result = cursor.fetchone()
    if result is None:
        cursor.close()
        # cnx.close()
        return False

    # If the user exists, update the reset code and expiry
    reset_expiry = datetime.datetime.now() + datetime.timedelta(hours=1)

    update_query = """
        UPDATE Users
        SET Reset_Code = %s,
            Reset_Expiry = %s
        WHERE Email = %s
    """
    params = (reset_code, reset_expiry.strftime('%Y-%m-%d %H:%M:%S'), user_email)
    try:
        cursor.execute(update_query, params)
        cnx.commit()
    except Exception as e:
        print(f"Error when trying to update reset code: {e}")
        cursor.close()
        # cnx.close()
        return False

    cursor.close()
    # cnx.close()

    return reset_code


def verify_password(cnx, username: str, password: str) -> bool:
    cursor = cnx.cursor()
    print('checking pw')
    cursor.execute("SELECT Hashed_PW FROM Users WHERE Username = %s", (username,))
    result = cursor.fetchone()
    cursor.close()

    if not result:
        return False  # User not found

    hashed_password = result[0]

    ph = PasswordHasher()
    try:
        # Attempt to verify the password
        ph.verify(hashed_password, password)
        # If verification does not raise an exception, password is correct
        # Optionally rehash the password if needed (argon2 can detect this)
        if ph.check_needs_rehash(hashed_password):
            new_hash = ph.hash(password)
            # Update database with new hash if necessary
            # You'll need to implement this part
            # update_hashed_password(cnx, username, new_hash)
        return True
    except VerifyMismatchError:
        # If verification fails, password is incorrect
        return False
    
def check_saved_session(cnx, session_value):
    cursor = cnx.cursor()
    cursor.execute("SELECT UserID, expire FROM Sessions WHERE value = %s", (session_value,))
    result = cursor.fetchone()
    cursor.close()

    if result:
        user_id, session_expire = result
        current_time = datetime.datetime.now()
        if current_time < session_expire:
            return user_id
    return None


def verify_reset_code(cnx, user_email, reset_code):
    cursor = cnx.cursor()

    select_query = """
        SELECT Reset_Code, Reset_Expiry
        FROM Users
        WHERE Email = %s
    """
    cursor.execute(select_query, (user_email,))
    result = cursor.fetchone()

    cursor.close()
    # cnx.close()

    # Check if a user with this email exists
    if result is None:
        return None

    # Check if the reset code is valid and not expired
    stored_code, expiry = result
    if stored_code == reset_code and datetime.datetime.now() < expiry:
        return True

    return False

def check_reset_user(cnx, username, email):
    cursor = cnx.cursor()
    query = "SELECT * FROM Users WHERE Username = %s AND Email = %s"
    cursor.execute(query, (username, email))
    result = cursor.fetchone()
    return result is not None


def reset_password_prompt(cnx, user_email, hashed_pw):
    cursor = cnx.cursor()

    update_query = """
        UPDATE Users
        SET Hashed_PW = %s,
            Reset_Code = NULL,
            Reset_Expiry = NULL
        WHERE Email = %s
    """
    params = (hashed_pw, user_email)
    cursor.execute(update_query, params)

    if cursor.rowcount == 0:
        return None

    cnx.commit()
    cursor.close()
    # cnx.close()

    return "Password Reset Successfully"

def save_mfa_secret(database_type, cnx, user_id, mfa_secret):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    query = (f"UPDATE Users "
             f"SET MFA_Secret = %s "
             f"WHERE UserID = %s")

    try:
        cursor.execute(query, (mfa_secret, user_id))
        cnx.commit()
        cursor.close()
        logging.info(f"Successfully saved MFA secret for user {user_id}")
        return True
    except Exception as e:
        logging.error(f"Error saving MFA secret for user {user_id}: {e}")
        return False

def check_mfa_enabled(database_type, cnx, user_id):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    query = (f"SELECT MFA_Secret FROM Users WHERE UserID = %s")

    try:
        cursor.execute(query, (user_id,))
        result = cursor.fetchone()
        cursor.close()

        # Check if MFA_Secret is NULL
        if result['MFA_Secret']:
            return True  # MFA is enabled
        else:
            return False  # MFA is disabled
    except Exception as e:
        print("Error checking MFA status:", e)
        return False


def get_mfa_secret(database_type, cnx, user_id):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    query = (f"SELECT MFA_Secret FROM Users WHERE UserID = %s")

    try:
        cursor.execute(query, (user_id,))
        result = cursor.fetchone()
        cursor.close()

        return result['MFA_Secret']
    except Exception as e:
        print("Error retrieving MFA secret:", e)
        return None


def delete_mfa_secret(database_type, cnx, user_id):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    query = (f"UPDATE Users SET MFA_Secret = NULL WHERE UserID = %s")

    try:
        cursor.execute(query, (user_id,))
        cnx.commit()
        cursor.close()

        return True
    except Exception as e:
        print("Error deleting MFA secret:", e)
        return False

def setup_timezone_info(database_type, cnx, user_id, timezone, hour_pref, date_format):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    query = f"""UPDATE Users SET Timezone = %s, TimeFormat = %s, DateFormat = %s, FirstLogin = %s WHERE UserID = %s"""

    try:
        cursor.execute(query, (timezone, hour_pref, date_format, 1, user_id))
        cnx.commit()
        cursor.close()

        return True
    except Exception as e:
        print("Error setting up time info:", e)
        return False


def get_time_info(database_type, cnx, user_id):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)
    query = (f"""SELECT Timezone, TimeFormat, DateFormat FROM Users WHERE UserID = %s""")

    cursor.execute(query, (user_id,))
    result = cursor.fetchone()
    cursor.close()

    if result:
        return result['Timezone'], result['TimeFormat'], result['DateFormat']
    else:
        return None, None, None


def first_login_done(database_type, cnx, user_id):
    if database_type == "postgresql":
        cursor = cnx.cursor(cursor_factory=RealDictCursor)
    else:  # Assuming MariaDB/MySQL if not PostgreSQL
        cursor = cnx.cursor(dictionary=True)

    # Query to fetch the FirstLogin status
    query = "SELECT FirstLogin FROM Users WHERE UserID = %s"

    try:
        # Execute the query
        cursor.execute(query, (user_id,))

        # Fetch the result
        result = cursor.fetchone()
        cursor.close()

        # Check if the FirstLogin value is 1
        if result['FirstLogin'] == 1:
            return True
        else:
            return False

    except Exception as e:
        print("Error fetching first login status:", e)
        return False

import time

def backup_server(cnx, database_pass):
    # Replace with your database and authentication details
    print(f'pass: {database_pass}')
    cmd = [
        "mysqldump",
        "-h", 'db',
        "-P", '3306',
        "-u", "root",
        "-p" + database_pass,
        "pypods_database"
    ]

    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    stdout, stderr = process.communicate()
    print("STDOUT:", stdout.decode())
    print("STDERR:", stderr.decode())

    if process.returncode != 0:
        # Handle error
        raise Exception(f"Backup failed with error: {stderr.decode()}")

    return stdout.decode()


def restore_server(cnx, database_pass, server_restore_data):
    import tempfile
    # Create a temporary file to store the content. This is because the mysql command reads from a file.
    with tempfile.NamedTemporaryFile(mode='w+', delete=True) as tempf:
        tempf.write(server_restore_data)
        tempf.flush()
        cmd = [
            "mysql",
            "-h", 'db',
            "-P", '3306',
            "-u", "root",
            "-p" + database_pass,
            "pypods_database"
        ]

        # Use the file's content as input for the mysql command
        with open(tempf.name, 'r') as file:
            process = subprocess.Popen(cmd, stdin=file, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            stdout, stderr = process.communicate()

            if process.returncode != 0:
                raise Exception(f"Restoration failed with error: {stderr.decode()}")

    return "Restoration completed successfully!"

def generate_access_key(length=32):
    import secrets
    characters = string.ascii_letters + string.digits
    return ''.join(secrets.choice(characters) for _ in range(length))


def create_scp_user(username, password, expiration_date):
    # Create user with home directory
    subprocess.run(["adduser", "-D", "-h", f"/home/{username}", username], check=True)
    # Set user password
    subprocess.run(f"echo '{username}:{password}' | chpasswd", shell=True, check=True)
    # Alpine doesn't support chage, handle expiration manually if necessary

def delete_scp_user(username):
    import subprocess
    subprocess.run(["userdel", "-r", username], check=True)

def get_config_info(cnx, config_id):
    cursor = cnx.cursor()
    try:
        # Fetch the configuration details
        query = """
        SELECT c.ConfigID, c.StorageLocation, s.Link, s.AccessKey
        FROM Configurations c
        LEFT JOIN SharedConfigs s ON c.ConfigID = s.ConfigID
        WHERE c.ConfigID = %s
        LIMIT 1
        """
        cursor.execute(query, (config_id,))
        result = cursor.fetchone()

        if result:
            config_id, storage_location, shared_link, access_key = result
            return {
                "config_id": config_id,
                "storage_location": storage_location,
                "shared_link": shared_link,
                "access_key": access_key
            }
        else:
            return None
    except Exception as e:
        print(f"Failed to get configuration info: {e}")
        return None
    finally:
        cursor.close()


def add_config_to_db(db, user_id, device_hostname, location, client_name, device_type, config_name, storage_location, file_path, url):
    from datetime import datetime, timedelta, timezone

    cursor = db.cursor()
    try:
        # Insert basic configuration data
        query = """
        INSERT INTO Configurations (UserID, DeviceHostname, ClientName, Location, DeviceType, ConfigName, StorageLocation)
        VALUES (%s, %s, %s, %s, %s, %s, %s)
        RETURNING ConfigID
        """
        cursor.execute(query, (user_id, device_hostname, client_name, location, device_type, config_name, storage_location))
        config_id = cursor.fetchone()[0]

        # Append the config_id to file path to create unique filename
        file_name = f"{config_id}.conf"
        full_file_path = os.path.join(file_path, file_name)

        # Update FilePath with the full file path
        update_query = """
        UPDATE Configurations
        SET FilePath = %s
        WHERE ConfigID = %s
        """
        cursor.execute(update_query, (full_file_path, config_id))

        # Generate access key and link
        access_key = generate_access_key()
        link = f"{url}/api/data/{config_id}/{access_key}"
        expires_at = datetime.now(timezone.utc) + timedelta(weeks=1)

        # Insert the shared configuration details
        query_insert_shared = """
        INSERT INTO SharedConfigs (ConfigID, Link, AccessKey, ExpiresAt)
        VALUES (%s, %s, %s, %s)
        """
        cursor.execute(query_insert_shared, (config_id, link, access_key, expires_at))

        db.commit()
        create_scp_user(str(config_id), access_key, expires_at)
        return config_id, link, access_key
    except Exception as e:
        db.rollback()
        print(f"Failed to add configuration: {e}")
        return None, None, None
    finally:
        cursor.close()

def edit_config(db, config_id):
    # Update the last updated time
    cursor = db.cursor()
    try:
        query = "UPDATE Configurations SET UpdatedAt = NOW() WHERE ConfigID = %s"
        cursor.execute(query, (config_id,))
        db.commit()
    except Exception as e:
        db.rollback()
        print(f"Failed to update configuration: {e}")


def get_shared_configuration(db, config_id, access_key):
    from datetime import datetime, timezone
    from dateutil import parser 
    query = """
    SELECT c.FilePath, s.ExpiresAt
    FROM Configurations c
    JOIN SharedConfigs s ON c.ConfigID = s.ConfigID
    WHERE s.ConfigID = %s AND s.AccessKey = %s
    LIMIT 1
    """
    cursor = db.cursor()
    try:
        cursor.execute(query, (config_id, access_key))
        config_info = cursor.fetchone()

        if not config_info:
            return None, "Shared configuration not found or expired"

        file_path, expires_at = config_info

        # Parse expires_at if it's a string
        if isinstance(expires_at, str):
            expires_at = parser.parse(expires_at)

        # Ensure expires_at is timezone aware
        if expires_at.tzinfo is None or expires_at.tzinfo.utcoffset(expires_at) is None:
            expires_at = expires_at.replace(tzinfo=timezone.utc)

        current_utc_time = datetime.now(timezone.utc)
        print(f"Current UTC Time: {current_utc_time}, Expiration Time: {expires_at}")

        if current_utc_time > expires_at:
            return None, "The shared configuration has expired"

        return file_path, None
    except Exception as e:
        return None, f"Database query error: {str(e)}"
    finally:
        cursor.close()

def get_configuration(cnx, config_id):
    query = """
    SELECT FilePath
    FROM Configurations
    WHERE ConfigID = %s
    LIMIT 1
    """
    cursor = cnx.cursor()
    try:
        cursor.execute(query, (config_id,))
        result = cursor.fetchone()
        if not result:
            return None
        return result[0], None
    except Exception as e:
        return None, f"Databse query error: {str(e)}"

def get_config_count(cnx):
    cursor = cnx.cursor()
    query = "SELECT COUNT(*) FROM Configurations"
    cursor.execute(query)
    count = cursor.fetchone()[0]
    cursor.close()
    return count

def get_config_count_user(cnx, user_id):
    cursor = cnx.cursor()
    query = "SELECT COUNT(*) FROM Configurations WHERE UserID = %s"
    cursor.execute(query, (user_id,))
    count = cursor.fetchone()[0]
    cursor.close()
    return count


def db_get_config_info(cnx, config_id):
    cursor = cnx.cursor()
    query = """SELECT ConfigID, UserID, DeviceHostname, ClientName, Location, DeviceType, ConfigName, StorageLocation, FilePath, CreatedAt, UpdatedAt
FROM Configurations WHERE ConfigID = %s"""
    cursor.execute(query, (config_id,))
    result = cursor.fetchone()
    cursor.close()
    return result

def get_config_list(cnx):
    cursor = cnx.cursor()
    query = """SELECT ConfigID, DeviceHostname, ClientName, Location, DeviceType, ConfigName, StorageLocation, FilePath, CreatedAt, UpdatedAt
FROM Configurations"""
    cursor.execute(query)
    result = cursor.fetchall()
    cursor.close()
    return result

def get_user_configs(cnx, user_id):
    cursor = cnx.cursor()
    query = """SELECT ConfigID, DeviceHostname, ClientName, Location, DeviceType, ConfigName, StorageLocation, FilePath, CreatedAt, UpdatedAt
FROM Configurations WHERE UserID = %s"""
    cursor.execute(query, (user_id,))
    result = cursor.fetchall()
    cursor.close()
    return result

def get_saved_configs(cnx, user_id):
    cursor = cnx.cursor()
    query = """
    SELECT c.ConfigID, c.DeviceHostname, c.ClientName, c.Location, c.DeviceType, c.ConfigName, c.StorageLocation, c.FilePath, c.CreatedAt, s.SavedAt
    FROM Configurations c
    JOIN SavedConfigurations s ON c.ConfigID = s.ConfigID
    WHERE s.UserID = %s
    """
    cursor.execute(query, (user_id,))
    result = cursor.fetchall()
    cursor.close()
    return result


def save_user_config(cnx, user_id, config_id):
    cursor = cnx.cursor()
    query = """
    INSERT INTO SavedConfigurations (UserID, ConfigID)
    VALUES (%s, %s)
    """
    try:
        cursor.execute(query, (user_id, config_id))
        cnx.commit()
    except Exception as e:
        cnx.rollback()
        raise e
    finally:
        cursor.close()

def remove_saved_user_config(cnx, user_id, config_id):
    cursor = cnx.cursor()
    query = """
    DELETE FROM SavedConfigurations WHERE UserID = %s AND ConfigID = %s
    """
    try:
        cursor.execute(query, (user_id, config_id))
        cnx.commit()
    except Exception as e:
        print(f"Error removing saved configuration: {e}")  # Add logging
        cnx.rollback()
        raise e
    finally:
        cursor.close()
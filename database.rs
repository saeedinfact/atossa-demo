use rusqlite::Connection;

//Implements User table structure
#[allow(dead_code)]
pub struct User {
    id: i32,       //PRIMARY
    name: String,  //UNIQUE
    email: String, //UNIQUE
    password: String,
}

//Constructs User object
#[allow(dead_code)]
impl User {
    pub fn new(id: i32, name: String, email: String, password: String) -> User {
        User {
            id,
            name,
            email,
            password,
        }
    }
}

//Implements Token table structure
#[allow(dead_code)]
pub struct Token {
    user_name: String, //UNIQUE
    ex_date: String,   //UNIQUE
    mac: String,       //PRIMARY
}

//Constructs Token object
#[allow(dead_code)]
impl Token {
    pub fn new(user_name: String, ex_date: String, mac: String) -> Token {
        Token {
            user_name,
            ex_date,
            mac,
        }
    }
}

//Implements Role table structure
#[allow(dead_code)]
pub struct Role {
    id: i32,      //PRIMARY
    name: String, //UNIQUE
}

//Implements UserRole table structure
#[allow(dead_code)]
pub struct UserRole {
    user_id: i32,
    role_id: i32,
}

//Includes necessary functions
#[allow(dead_code)]
pub struct Query;

//Constructs Query object
#[allow(dead_code)]
impl Query {
    pub fn new() -> Query {
        Query {

        }
    }

    //Executes the query
    pub fn perform(&self, conn: Connection, query: String) {
        match conn.execute(&query, &[]) {
            Ok(_value) => (),
            Err(msg) => panic!("Query execution error: {}", msg),
        }
    }

    //Inserts a users into User table
    pub fn insert_user(&self, conn: Connection, user: User) {
        let query = "INSERT INTO user (user_id, user_name, user_email, user_password)
                  VALUES (?1, ?2, ?3, ?4)";
        match conn.execute(query, &[&user.id, &user.name, &user.email, &user.password]) {
            Ok(_value) => (),
            Err(msg) => panic!("User insertion error: {}", msg),
        }
    }

    //Verifies login for a certain user
    pub fn login(&self, conn: Connection, user: User) -> bool {
        let query = "SELECT user_id, user_name, user_email, user_password FROM user";

        //Outputs the query into stmt
        let mut stmt = match conn.prepare(query) {
            Ok(value) => value,
            Err(msg) => panic!("Statement preparation error: {}", msg),
        };

        //Creates an iterator for the query
        let user_iter = match stmt.query_map(&[], |row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            password: row.get(3),
        }) {
            Ok(value) => value,
            Err(msg) => panic!("Iterator creation error: {}", msg),
        };


        //Finds user in user_iter
        for element in user_iter {
            let candidate = match element {
                Ok(value) => value,
                Err(msg) => panic!("User iteration error: {}", msg),
            };
            if candidate.name == user.name {
                if candidate.password == user.password {
                    println!("Login successful!");
                    return true;
                }
            }
        }
        println!("Login failed!");
        return false;
    }

    //Inserts a token into Token table
    pub fn insert_token(&self, conn: Connection, token: Token) {
        let query = "INSERT INTO tokens_table (user_name, ex_date, mac)
                  VALUES (?1, ?2, ?3)";

        //Executes the query
        match conn.execute(query, &[&token.user_name, &token.ex_date, &token.mac]) {
            Ok(_value) => (),
            Err(msg) => panic!("Query execution error: {}", msg),
        }
    }

    //Verifies access for a certain MAC phrase
    pub fn have_access(&self, conn: Connection, mac: String) -> bool {
        let query = "SELECT user_name, ex_date, mac FROM tokens_table";

        //Outputs the query into stmt
        let mut stmt = match conn.prepare(query) {
            Ok(value) => value,
            Err(msg) => panic!("Statement preparation error: {}", msg),
        };

        //Creates an iterator for the query
        let user_iter = match stmt.query_map(&[], |row| Token {
            user_name: row.get(0),
            ex_date: row.get(1),
            mac: row.get(2),
        }) {
            Ok(value) => value,
            Err(msg) => panic!("Iterator creation error: {}", msg),
        };

        //Finds user in user_iter
        for element in user_iter {
            let candidate = match element {
                Ok(value) => value,
                Err(msg) => panic!("User iteration error: {}", msg),
            };
            if candidate.mac == mac {
                println!("Access verified!");
                return true;
            }
        }
        println!("Access denied!");
        return false;
    }

    //Returns information of a certain username
    pub fn user_info(&self, conn: Connection, user_name: String) -> (i32, String, String, bool) {
        let query = "SELECT user_id, user_name, user_email, user_password FROM user";

        //Outputs the query into stmt
        let mut stmt = match conn.prepare(query) {
            Ok(value) => value,
            Err(msg) => panic!("Statement preparation error: {}", msg),
        };

        //Creates an iterator for the query
        let user_iter = match stmt.query_map(&[], |row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            password: row.get(3),
        }) {
            Ok(value) => value,
            Err(msg) => panic!("Iterator creation error: {}", msg),
        };

        //Finds user in user_iter
        for element in user_iter {
            let candidate = match element {
                Ok(value) => value,
                Err(msg) => panic!("User iteration error: {}", msg),
            };
            if candidate.name == user_name {
                return (
                    candidate.id,
                    candidate.name,
                    candidate.email,
                    true,
                );
            }
        }
        return (
            0,
            "not found".to_string(),
            "not found".to_string(),
            false
        );
    }

    pub fn mail_check(&self, conn: Connection, email: String) -> bool {
        let query = "SELECT user_id, user_name, user_email, user_password FROM user";

        //Outputs the query into stmt
        let mut stmt = match conn.prepare(query) {
            Ok(value) => value,
            Err(msg) => panic!("Statement preparation error: {}", msg),
        };

        //Creates an iterator for the query
        let user_iter = match stmt.query_map(&[], |row| User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
            password: row.get(3),
        }) {
            Ok(value) => value,
            Err(msg) => panic!("Iterator creation error: {}", msg),
        };

        //Finds user in user_iter
        for element in user_iter {
            let candidate = match element {
                Ok(value) => value,
                Err(msg) => panic!("User iteration error: {}", msg),
            };
            if candidate.email == email {
                    return true;
            }
        }
        return false;
    }
}

/*
extern crate rusqlite;
use rusqlite::Connection;

mod database;
use database::Query;
use database::User;

fn main() {
    let path = "./DEV_DB_TEST";
    let conn = Connection::open(path).unwrap();
    let query = Query::new();


    let user_table =
    "CREATE TABLE user (
        user_id              INTEGER PRIMARY KEY,
        user_name            STRING UNIQUE,
        user_email           STRING UNIQUE,
        user_password        STRING
    )".to_string();

    let role_table =
    "CREATE TABLE role (
        role_id              INTEGER PRIMARY KEY,
        role_name            STRING UNIQUE
    )".to_string();

    let user_role =
    "CREATE TABLE role (
        user_id              INTEGER PRIMARY KEY,
        role_id            INTEGER PRIMARY KEY
    )".to_string();



    let user = User::new(
        12,
        "admin".to_string(),
        "admin@gmail.com".to_string(),
        "admin".to_string()
    );

}
*/

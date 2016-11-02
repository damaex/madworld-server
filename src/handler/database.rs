extern crate mysql;

use std::fmt;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    LEFT = 0,
    UP = 1,
    RIGHT = 2,
    DOWN = 3,
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
    direction: Direction,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "(X: {}, Y: {}, Direction: {:?})", self.x, self.y, self.direction)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct User {
    id: i32,
    mail: String,
    name: String,
    position: Position,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "(ID: {}, Mail: {}, Name: {}, Position: {})", self.id, self.mail, self.name, self.position)
    }
}

pub struct DatabaseWorker {
    pool:       mysql::Pool,
}

impl DatabaseWorker {
    pub fn new(host: String, database: String, user: String, password: String, port: u16) -> DatabaseWorker {
        let mut builder = mysql::OptsBuilder::new();

        builder.ip_or_hostname(Some(host))
            .db_name(Some(database))
            .user(Some(user))
            .pass(Some(password))
            .tcp_port(port);

        DatabaseWorker {
            pool: mysql::Pool::new(builder).unwrap(),
        }
    }

    fn login(&self, mail: String, key: String) -> User {
        User {
            id: 0,
            mail: mail,
            name: key,
            position: Position {
                x: 0,
                y: 0,
                direction: Direction::DOWN,
            }
        }
    }
}
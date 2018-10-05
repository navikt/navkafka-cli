extern crate clap;
extern crate ldap3;

use clap::{App, Arg, SubCommand};
use ldap3::{LdapConn};

use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let matches = App::new("navkafka-cli")
        .version("0.1")
        .author("Kevin Sillerud <kevin.sillerud@nav.no>")
        .about("Application for doing operations against NAVs kafka environment")
        .subcommand(SubCommand::with_name("ldap")
            .about("Subcommand for doing ldap operations against the docker-compose kafka environment")
            .arg(Arg::with_name("ldap-url")
                .short("h")
                .long("ldap-url")
                .help("The url for the ldap server")
                .default_value("ldap://localhost:10389")
            )
            .arg(Arg::with_name("bind-dn")
                .short("u")
                .long("bind-dn")
                .help("dn string to use for bind")
                .default_value("cn=igroup,ou=ServiceAccounts,dc=test,dc=local")
            )
            .arg(Arg::with_name("bind-password")
                .short("p")
                .long("bind-password")
                .help("Password to use for binding against ldap")
                .default_value("itest")
            )
            .subcommand(SubCommand::with_name("useradd")
                .about("Add a user")
                .arg(Arg::with_name("user-type")
                    .possible_values(&["SERVICE_USER", "PERSONAL_USER"])
                    .short("t")
                    .long("user-type")
                    .help("Define the type of user to add")
                    .default_value("SERVICE_USER")
                )
                .arg(Arg::with_name("username")
                    .required(true)
                    .index(1)
                )
                .arg(Arg::with_name("password")
                    .required(true)
                    .index(2)
                )
            )
        )
        .get_matches();

    if let Some(ldap_command) = matches.subcommand_matches("ldap") {
        let ldap_url = ldap_command.value_of("ldap-url").unwrap();
        let bind_dn = ldap_command.value_of("bind-dn").unwrap();
        let bind_password = ldap_command.value_of("bind-password").unwrap();

        let ldap = LdapConn::new(ldap_url).expect("Failed to connect to LDAP server");
        ldap.simple_bind(bind_dn, bind_password).unwrap();
        if let Some(add_command) = ldap_command.subcommand_matches("useradd") {
            let username = add_command.value_of("username").unwrap();
            let password = add_command.value_of("password").unwrap();

            let dn = if add_command.value_of("user-type").unwrap() == "SERVICE_USER" {
                format!("cn={},ou=ApplAccounts,ou=ServiceAccounts,dc=test,dc=local", username)
            } else {
                format!("cn={},ou=Users,ou=NAV,ou=BusinessUnits,dc=test,dc=local", username)
            };
            println!("{}", dn);
            ldap.add(dn.as_ref(), vec![
                ("objectClass", HashSet::from_iter(vec!["inetOrgPerson", "person", "organizationalPerson"])),
                ("cn", HashSet::from_iter(vec![username.as_ref()])),
                ("uid", HashSet::from_iter(vec![username.as_ref()])),
                ("sn", HashSet::from_iter(vec![username.as_ref()])),
                ("userPassword", HashSet::from_iter(vec![password]))
            ]).unwrap();
        }
    }
}

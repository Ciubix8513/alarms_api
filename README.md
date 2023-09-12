# alarms_api
### Overview
In self hosted enviorments not all servers are able to have the necessary hardware and software to alert if something goes wrong, however comonly, all systems are able to send web requests. This api uses that to centralize the response to alarms, raised by various hosts in the network.
Using this api, only one host has to have the necessary alertion hardware and software.

This api tries to adhere the unix philosophy and therefore doesn't have any detection systems of its own, only the responses.

## Usage
The api has 2 endpoints `/alarm` and `/disable_alarm`
### /alarm
This endpoint starts a new alarm according to the config, it takes in the following data structure:
```Rust
api_key : String
host_id : String
severity : Severity
failure_cause : String
```
Severity can be one ofthe following:
```Rust
Low,
Middle,
High,
Test
```
If the created alarm is repeated (see config), it generates an alarm id which is logged.
This alarm id is the only way to disable an alarm, this is intentional. This way one can not automate the disabling of repeated alarms.
### /disable_alarm
This endpoint disables one alarm, if it's repeating, it takes in the following data structure: 
```Rust
api_key : String
id : u32
```
It disables the repeated alarm with a given id, this endpoint is meant to be pinged manually.

## Config
All responses are configured using a config, you can generate a sample config by passing a `-g` flag when running the api.
Here's how it looks:
```yaml
hosts:
- name: Host
  responses:
  - severity: Low
    response: !Log Log error
    repeating: null
  - severity: Middle
    response: !Sound
      file_path: ~/Music/test.mp3
      run_directory: null
    repeating: null
  - severity: High
    response: !File
      file_path: ~/test.sh
      run_directory: ~/
    repeating:
      secs: 1
      nanos: 0
api_key: '123'
ip_address: 127.0.0.1
port: 5000
```
The main part of the config is a list of hosts, each with their unique name, if a name is not unique, only the first set of responses will be used.
Each host has a list of responses. Every response consist, of severity, response type, and repeating status.

### Response types
#### Log
Logs a specified message to the api log
#### Sound
Plays a specified sound file untill it finishes
#### File
Executes a specified file, in a specified run directory
### Repeating status
It is possible to make an alarm repeating, this way it will do the specified repetedly with a specified time period.

### Additional settings
The config file also specifies the api key, ip address and the port to which the api binds. Please **do not** use the default api key 

## Running the api 
To run the api, simply compile it using the latest rustc version. When running the following options are available:
```
  -c, --config-path <CONFIG_PATH>  Path to the config file [default: ./config.yaml]
  -g, --generate-config            If set will generate the default config file at the provided config path
  -h, --help                       Print help
```

### Additional note
This api was built entirely live on [twitch](https://twitch.tv/ciubix8513) , there's also a [playlist](https://youtube.com/playlist?list=PL1A2-mjp7jA8KCW6hZ-fpHMjtlOOfcjKV&si=zrLjoVKiRob5baAN) of all the VODs of the api development 

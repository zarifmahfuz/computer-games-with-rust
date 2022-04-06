# Backend

## Build

### Requirements

The backend will be running on your localhost. So, all data will be stored in a database hosted on your local machine. We are using MongoDB. Please make sure that you have [MongoDB Version 3.6+](https://www.mongodb.com/docs/manual/installation/) installed and running on your computer. The API server will be using the address `mongodb://localhost:27017` to connect to the MongoDB cluster.

### Run

Ensure that you are currently in the `backend` directory of this project. Then,
```
cargo run
```

The API server is running on port 5000. If you want to change that, please edit the `Rocket.toml` file.

## API

### API Endpoints
| Resource                  | POST                    |     GET     |  DELETE |
| ----------------------------------- |:-------------:| :-----:| :-----:|
| /gameresults              | Creates a new game result | Retrieves all game results     | Deletes all game results |
| /gameresults?winner_name | - | Retrieves all game results for a particular winner     | - |
| /leaderboard              | - | Retrieves the leaderboard for all difficulties          | - |
| /leaderboard?difficulty | - | Retrieves the leaderboard for a particular difficulty level     | - |

### Examples

#### Create a new game result

```
curl -X POST -H "Content-Type: application/json" http://127.0.0.1:5000/gameresults -d '{
    "game_type": "Connect-4",
    "p1_name": "Zarif",
    "p2_name": "Computer",
    "is_draw": false,
    "winner_name": "Computer",
    "difficulty": "Hard",
    "date_time": "2022-04-06T05:00:00.00Z"
}'
```

Response:
```
"9"
```

**Note 1**: You must specify all the fields as shown here. For the `game_type` field, please enter either of `Connect-4` or `TootAndOtto`. For the `difficulty` field, please enter either of "Easy", "Medium" or "Hard". Ensure that the `winner_name` field matches with either the `p1_name` or the `p2_name` field.

**Note 2**: You will need to specify the `date_time` field as an [RFC3339](https://datatracker.ietf.org/doc/html/rfc3339) formatted string.

**Note 3**: In the above response, `9` is the id of the newly created document.

#### Retrieve all game results for a particular winner

```
curl -X GET "http://127.0.0.1:5000/gameresults?winner_name=Zarif"
```

Response:
```
[{"_id":"4","game_type":"Connect-4","p1_name":"Zarif","p2_name":"Computer","is_draw":false,"winner_name":"Zarif","difficulty":"Hard","date_time":"2022-04-06T05:00:00Z"},{"_id":"5","game_type":"Connect-4","p1_name":"Zarif","p2_name":"Computer","is_draw":false,"winner_name":"Zarif","difficulty":"Medium","date_time":"2022-04-06T05:00:00Z"},{"_id":"6","game_type":"Connect-4","p1_name":"Zarif","p2_name":"Computer","is_draw":false,"winner_name":"Zarif","difficulty":"Medium","date_time":"2022-04-06T05:00:00Z"}]
```

#### Retrieve the leaderboard for all difficulty levels

```
curl -X GET "http://127.0.0.1:5000/leaderboard"
```

Response:
```
[{"winner_name":"Computer","wins":6},{"winner_name":"Zarif","wins":3}]
```

#### Retrieve the leaderboard for a particular difficulty level

```
curl -X GET "http://127.0.0.1:5000/leaderboard?difficulty=Medium"
```

Response:
```
[{"winner_name":"Zarif","wins":2},{"winner_name":"Computer","wins":1}]
```

As you can see, the leaderboard is sorted in descending order of wins.

## single file:

#### example.json
```json
{
	"value": 1,
	"right": {
		"value": 2,
		"right": {
			"value": 4,
			"right": {
				"value": 8
			},
			"left": {
				"value": 9
			}
		},
		"left": {
			"value": 5,
			"right": {
				"value": 10
			},
			"left": {
				"value": 11
			}
		}
	},
	"left": {
		"value": 3,
		"right": {
			"value": 6,
			"right": {
				"value": 12
			},
			"left": {
				"value": 13
			}
		},
		"left": {
			"value": 7,
			"right": {
				"value": 14
			},
			"left": {
				"value": 15
			}
		}
	}
}
```
#### output
![output](https://i.imgur.com/pZsdZY2.png)


## multiple files:

#### first.json
```json
{
	"value": 1,
	"right": {
		"value": 2,
		"left": {
			"value": 4,
			"left": {
				"value": 6,
				"right": {
					"value": 8
				},
				"left": {
					"value": 9
				}
			}
		}
	},
	"left": {
		"value": 3,
		"right": {
			"value": 5,
			"right": {
				"value": 7,
				"right": {
					"value": 10
				},
				"left": {
					"value": 11
				}
			}
		}
	}
}
```
#### second.json
```json
{
	"value": 1,
	"right": {
		"value": 2,
		"right": {
			"value": 4,
			"right": {
				"value": 6,
				"right": {
					"value": 8
				},
				"left": {
					"value": 9
				}
			}
		}
	},
	"left": {
		"value": 3,
		"left": {
			"value": 5,
			"left": {
				"value": 7,
				"right": {
					"value": 10
				},
				"left": {
					"value": 11
				}
			}
		}
	}
}
```
#### output
![](https://i.imgur.com/9mJ8Rp4.png)

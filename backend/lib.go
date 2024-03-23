package main

type Session struct {
	ID          int    `json:"id"`
	SessionType string `json:"session_type"`
}

type Event struct {
	ID        int    `json:"id"`
	SessionID int    `json:"session_id"`
	Timestamp string `json:"timestamp"`
	Capacity  int    `json:"capacity"`
	PowerDraw int    `json:"power_draw"`
}

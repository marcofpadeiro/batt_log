package main

import (
	"log"
	"time"
)

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

type DetailedSession struct {
	ID                        int    `json:"id"`
	SessionType               string `json:"session_type"`
	Start                     string `json:"start"`
	End                       string `json:"end"`
	Duration                  int    `json:"duration"`
	Capacity_Start            int    `json:"capacity_start"`
	Capacity_End              int    `json:"capacity_end"`
	Capacity_Delta            int    `json:"capacity_delta"`
	Average_Power_Consumption int    `json:"average_power_consumption"`
}

func calculateDuration(start string, end string) int {
	const timeLayout = "2006-01-02T15:04:05Z" // ISO 8601 format; change this to match your actual time format
	startTime, err := time.Parse(timeLayout, start)
	if err != nil {
		log.Printf("Error parsing start time: %v\n", err)
		return -1
	}
	endTime, err := time.Parse(timeLayout, end)
	if err != nil {
		log.Printf("Error parsing end time: %v\n", err)
		return -1
	}

	duration := endTime.Sub(startTime)
	return int(duration.Seconds())
}

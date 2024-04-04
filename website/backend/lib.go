package main

import (
	"database/sql"
	"log"
	"net/http"
	"time"

	"github.com/gin-gonic/gin"
)

type Session struct {
	ID          int     `json:"id"`
	SessionType string  `json:"session_type"`
	Events      []Event `json:"events"`
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

func parseRowsIntoEvents(rows *sql.Rows) ([]Event, error) {
	var events []Event

	for rows.Next() {
		var temp Event
		if err := rows.Scan(&temp.ID, &temp.SessionID, &temp.Timestamp, &temp.Capacity, &temp.PowerDraw); err != nil {
			return nil, err
		}
		events = append(events, temp)
	}

	return events, nil
}

func parseRowsIntoDetailedSessions(rows *sql.Rows) ([]DetailedSession, error) {
	var detailedSessions []DetailedSession

	sessionsMap, err := mapEventsToSessions(rows)
	if err != nil {
		return nil, err
	}

	for _, session := range sessionsMap {
		detailedSessions = append(detailedSessions, convertSessionToDetailedSession(session))
	}

	return detailedSessions, nil
}

func mapEventsToSessions(rows *sql.Rows) (map[int]*Session, error) {
	sessionsMap := make(map[int]*Session)

	for rows.Next() {
		var event Event
		var session Session

		if err := rows.Scan(&event.ID, &event.SessionID, &event.Timestamp, &event.Capacity, &event.PowerDraw, &session.ID, &session.SessionType); err != nil {
			return nil, err
		}

		if tempSession, exists := sessionsMap[session.ID]; exists {
			tempSession.Events = append(tempSession.Events, event)
		} else {
			newSession := &Session{
				ID:          session.ID,
				SessionType: session.SessionType,
				Events:      []Event{event},
			}
			sessionsMap[session.ID] = newSession
		}
	}

	return sessionsMap, nil
}

func convertSessionToDetailedSession(session *Session) DetailedSession {
	return DetailedSession{
		ID:                        session.ID,
		SessionType:               session.SessionType,
		Start:                     session.Events[0].Timestamp,
		End:                       session.Events[len(session.Events)-1].Timestamp,
		Duration:                  calculateDuration(session.Events[0].Timestamp, session.Events[len(session.Events)-1].Timestamp),
		Capacity_Start:            session.Events[0].Capacity,
		Capacity_End:              session.Events[len(session.Events)-1].Capacity,
		Capacity_Delta:            session.Events[len(session.Events)-1].Capacity - session.Events[0].Capacity,
		Average_Power_Consumption: calculateAveragePower(session.Events),
	}
}

func calculateDuration(start string, end string) int {
	const timeLayout = "2006-01-02T15:04:05Z"
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

func calculateAveragePower(events []Event) int {
	totalPower := 0
	for _, event := range events {
		totalPower += event.PowerDraw
	}

	return totalPower / len(events)
}

func handleSingleQueryErr(err error, c *gin.Context) {
	if err == sql.ErrNoRows {
		c.JSON(http.StatusNotFound, gin.H{"error": "Event not found"})
		return
	} else if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
		return
	}
}

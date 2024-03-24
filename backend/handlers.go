package main

import (
	"database/sql"
	"log"
	"net/http"

	"github.com/gin-gonic/gin"
	_ "github.com/mattn/go-sqlite3"
)

func getSessions(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		// Query to get all sessions
		rows, err := db.Query("SELECT id, session_type FROM session ORDER BY id DESC")
		if err != nil {
			log.Printf("Error querying sessions: %v\n", err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		defer rows.Close()

		var sessions []Session

		for rows.Next() {
			var s Session
			if err := rows.Scan(&s.ID, &s.SessionType); err != nil {
				log.Printf("Error scanning sessions: %v\n", err)
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}
			sessions = append(sessions, s)
		}

		var detailedSessions []DetailedSession

		for _, session := range sessions {
			var events []Event
			eventRows, err := db.Query("SELECT * FROM event WHERE session_id = ? ORDER BY timestamp ASC", session.ID)
			if err != nil {
				log.Printf("Error querying events: %v\n", err)
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}

			var totalPower int

			for eventRows.Next() {
				var e Event
				if err := eventRows.Scan(&e.ID, &e.SessionID, &e.Timestamp, &e.Capacity, &e.PowerDraw); err != nil {
					log.Printf("Error scanning events: %v\n", err)
					eventRows.Close() // Close the eventRows before returning
					c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
					return
				}
				totalPower += e.PowerDraw
				events = append(events, e)
			}
			eventRows.Close()

			if len(events) > 0 {
				start := events[0].Timestamp
				end := events[len(events)-1].Timestamp
				duration := calculateDuration(start, end)
				capacityStart := events[0].Capacity
				capacityEnd := events[len(events)-1].Capacity
				capacityDelta := capacityEnd - capacityStart
				averagePower := totalPower / len(events)

				detailedSession := DetailedSession{
					ID:                        session.ID,
					SessionType:               session.SessionType,
					Start:                     start,
					End:                       end,
					Duration:                  duration,
					Capacity_Start:            capacityStart,
					Capacity_End:              capacityEnd,
					Capacity_Delta:            capacityDelta,
					Average_Power_Consumption: averagePower,
				}

				detailedSessions = append(detailedSessions, detailedSession)
			}
		}

		c.JSON(http.StatusOK, detailedSessions)
	}
}

func getSession(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		var session Session
		id := c.Param("id")

		err := db.QueryRow("SELECT * FROM session WHERE id = ?", id).Scan(&session.ID, &session.SessionType)

		if err == sql.ErrNoRows {
			c.JSON(http.StatusNotFound, gin.H{"error": "Session not found"})
			return
		} else if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		c.JSON(http.StatusOK, session)
	}
}

func getEvents(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		rows, err := db.Query("SELECT * FROM event")
		if err != nil {
			log.Fatal(err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		defer rows.Close()

		var events []Event

		for rows.Next() {
			var temp Event
			if err := rows.Scan(&temp.ID, &temp.SessionID, &temp.Timestamp, &temp.Capacity, &temp.PowerDraw); err != nil {
				log.Fatal(err)
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}
			events = append(events, temp)
		}

		c.JSON(http.StatusOK, events)
	}
}

func getEvent(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		var event Event
		id := c.Param("id")

		err := db.QueryRow("SELECT * FROM event WHERE id = ?", id).Scan(&event.ID, &event.SessionID, &event.Timestamp, &event.Capacity, &event.PowerDraw)

		if err == sql.ErrNoRows {
			c.JSON(http.StatusNotFound, gin.H{"error": "Event not found"})
			return
		} else if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		c.JSON(http.StatusOK, event)
	}
}

func getSessionEvents(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		id := c.Param("id")
		rows, err := db.Query("SELECT * FROM event WHERE session_id = ?", id)
		if err != nil {
			log.Fatal(err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		defer rows.Close()

		var events []Event

		for rows.Next() {
			var temp Event
			if err := rows.Scan(&temp.ID, &temp.SessionID, &temp.Timestamp, &temp.Capacity, &temp.PowerDraw); err != nil {
				log.Fatal(err)
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}
			events = append(events, temp)
		}

		c.JSON(http.StatusOK, events)
	}
}

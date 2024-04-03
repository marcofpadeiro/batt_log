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
		rows, err := db.Query("SELECT * FROM events INNER JOIN sessions ON events.session_id = sessions.id ORDER BY events.timestamp ASC")
		if err != nil {
			log.Printf("Error querying sessions: %v\n", err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		defer rows.Close()

		detailedSessions, err := parseRowsIntoDetailedSessions(rows)
		if err != nil {
			log.Printf("Error parsing rows into sessions: %v\n", err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		c.JSON(http.StatusOK, detailedSessions)
	}
}

func getEvents(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		rows, err := db.Query("SELECT * FROM events")
		if err != nil {
			log.Fatal(err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		defer rows.Close()

		events, err := parseRowsIntoEvents(rows)

		c.JSON(http.StatusOK, events)
	}
}

func getSession(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		var session Session
		id := c.Param("id")

		err := db.QueryRow("SELECT * FROM sessions WHERE id = ?", id).Scan(&session.ID, &session.SessionType)
		handleSingleQueryErr(err, c)

		c.JSON(http.StatusOK, session)
	}
}

func getEvent(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		var event Event
		id := c.Param("id")

		err := db.QueryRow("SELECT * FROM events WHERE id = ?", id).Scan(&event.ID, &event.SessionID, &event.Timestamp, &event.Capacity, &event.PowerDraw)
		handleSingleQueryErr(err, c)

		c.JSON(http.StatusOK, event)
	}
}

func getSessionEvents(db *sql.DB) gin.HandlerFunc {
	return func(c *gin.Context) {
		id := c.Param("id")
		rows, err := db.Query("SELECT * FROM events WHERE session_id = ?", id)
		if err != nil {
			log.Fatal(err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		defer rows.Close()

		events, err := parseRowsIntoEvents(rows)
		if err != nil {
			log.Fatal(err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}

		c.JSON(http.StatusOK, events)
	}
}

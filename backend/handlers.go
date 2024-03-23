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
		rows, err := db.Query("SELECT * FROM session")
		if err != nil {
			log.Fatal(err)
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		defer rows.Close()

		var sessions []Session

		for rows.Next() {
			var temp Session
			if err := rows.Scan(&temp.ID, &temp.SessionType); err != nil {
				log.Fatal(err)
				c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
				return
			}
			sessions = append(sessions, temp)
		}

		c.JSON(http.StatusOK, sessions)
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

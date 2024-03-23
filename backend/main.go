package main

import (
	"database/sql"
	"log"
	"os"

	"github.com/gin-gonic/gin"
)

func main() {
	db, err := sql.Open("sqlite3", os.Getenv("HOME")+"/.cache/batt.db")

	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	r := gin.Default()
	r.GET("/sessions", getSessions(db))
	r.GET("/sessions/:id", getSession(db))
	r.GET("/sessions/:id/events", getSessionEvents(db))
	r.GET("/events", getEvents(db))
	r.GET("/events/:id", getEvent(db))

	r.Run()
}

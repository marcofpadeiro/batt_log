package main

import (
	"database/sql"
	"github.com/gin-gonic/gin"
	"log"
)

func main() {
	config := NewConfig()

	db, err := sql.Open("sqlite3", config.DBPath)
	if err != nil {
		log.Fatal(err)
	}
	defer db.Close()

	r := gin.Default()

	r.Use(CORSMiddleware())

	r.GET("/sessions", getSessions(db))
	r.GET("/sessions/:id", getSession(db))
	r.GET("/sessions/:id/events", getSessionEvents(db))
	r.GET("/events", getEvents(db))
	r.GET("/events/:id", getEvent(db))

	r.Run()
}

package main

import (
	"encoding/json"
	"fmt"
	"log"
	"net/http"
)

type CalculationResponse struct {
	Message string `json:"message"`
	ReceivedDate string `json:"received_date"`
	ReceivedTime string `json:"received_time"`
	YearPillar string `json:"year_pillar"`
	MonthPillar string `json:"month_pillar"`
	DayPillar string `json:"day_pillar"`
	TimePillar string `json:"time_pillar"`
}

func askEngineJSON(w http.ResponseWriter, r *http.Request) {
	userDate := r.URL.Query().Get("date")
	userTime := r.URL.Query().Get("time")

	if userDate == "" {	userDate = "2000-01-01" }
	if userTime == "" {	userTime = "12:00" }

	targetURL := fmt.Sprintf("http://engine:8080/json?date=%s&time=%s", userDate, userTime)
	
	resp, err := http.Get(targetURL)
	if err != nil {
		http.Error(w, fmt.Sprintf("Error contacting engine: %v", err), http.StatusInternalServerError)
		return
	}
	defer resp.Body.Close()

	var result CalculationResponse
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		http.Error(w, "Failed to parse JSON", http.StatusInternalServerError)
		return
	}

	fmt.Fprintf(w, "--- Engine Status ---\n")
	fmt.Fprintf(w, "[Input Date] : %s\n", result.ReceivedDate)
	fmt.Fprintf(w, "[Input Time] : %s\n", result.ReceivedTime)
	fmt.Fprintf(w, "[Year Pillar] : %s (The Year's Fate)\n", result.YearPillar)
	fmt.Fprintf(w, "[Month Pillar] : %s (The Month's Fate)\n", result.MonthPillar)
	fmt.Fprintf(w, "[Day Pillar] : %s (The Day's Fate)\n", result.DayPillar)
	fmt.Fprintf(w, "[Time Pillar] : %s (The Time's Fate)\n", result.TimePillar)
	fmt.Fprintf(w, "[Message] : %s\n", result.Message)
}

func main() {
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		fmt.Fprintf(w, "Sanctuary Backend.\nTry: /json?date=2000-01-01&time=12:00")
	})

	http.HandleFunc("/json", askEngineJSON)

	log.Println("Backend starting on port 8000 ...")
	if err := http.ListenAndServe(":8000", nil); err != nil {
		log.Fatal(err)
	}
}
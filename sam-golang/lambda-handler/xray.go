package main

import (
	"encoding/json"
	"os"
	"strings"

	"github.com/aws/aws-sdk-go/service/xray"
)

type XrayServiceInfo struct {
	ResourceName string `json:"Name"`
	Duration     []struct {
		Count int     `json:"Count"`
		Value float32 `json:"Value"`
	} `json:"DurationHistogram"`
	ResourceType string `json:"Type"`
}

func getCurrentTracingID() string {
	tracingEnvVar := os.Getenv("_X_AMZN_TRACE_ID")
	tracingIdRoot := strings.Split(tracingEnvVar, ";")[0]
	tracingId := strings.Split(tracingIdRoot, "=")[1]

	return tracingId
}

func getTracesInfo(client *xray.XRay, traceId string) []XrayServiceInfo {
	graph, _ := client.GetTraceGraph(&xray.GetTraceGraphInput{
		TraceIds: []*string{&traceId},
	})

	services := graph.Services
	var servicesParsed []XrayServiceInfo

	// info := XrayServiceInfo{}
	// json.Unmarshal(data, v)
	res_json, _ := json.Marshal(services)
	json.Unmarshal(res_json, &servicesParsed)

	return servicesParsed
}

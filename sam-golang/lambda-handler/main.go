package main

import (
	"context"
	"encoding/json"
	"fmt"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	xrayApi "github.com/aws/aws-sdk-go/service/xray"
	"github.com/aws/aws-xray-sdk-go/xray"
)

func main() {
	lambda.Start(lambdaHandler)
}

func lambdaHandler(ctx context.Context, request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	sess := session.Must(session.NewSessionWithOptions(session.Options{
		SharedConfigState: session.SharedConfigEnable,
	}))

	tableName := "student-db"
	dynoClient := dynamodb.New(sess)
	xray.AWS(dynoClient.Client)

	currXrayTraceId := getCurrentTracingID()
	xrayClient := xrayApi.New(sess)

	fmt.Println("Tracing id: " + getCurrentTracingID())

	var APIResponse events.APIGatewayProxyResponse

	switch request.HTTPMethod {
	case "GET":
		APIResponse = getStudent(ctx, request, *dynoClient, tableName)

	case "PUT":
		APIResponse = putStudent(ctx, request, *dynoClient, tableName)

	case "DELETE":
		APIResponse = deleteStudent(ctx, request, *dynoClient, tableName)

	case "POST":
		APIResponse = updateStudent(ctx, request, *dynoClient, tableName)

	default:
		APIResponse = events.APIGatewayProxyResponse{Body: "Method not allowed", StatusCode: 405}
	}

	xrayData, _ := json.Marshal(getTracesInfo(xrayClient, currXrayTraceId))
	if APIResponse.StatusCode == 200 {
		APIResponse.Body = string(xrayData)
	}

	return APIResponse, nil
}

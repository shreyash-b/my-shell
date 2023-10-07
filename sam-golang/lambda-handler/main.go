package main

import (
	"context"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-xray-sdk-go/xray"
)

func main() {
	lambda.Start(lambdaHandler)
}

func lambdaHandler(ctx context.Context, request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	sess := session.Must(session.NewSessionWithOptions(session.Options{
		SharedConfigState: session.SharedConfigEnable,
	}))

	dynoClient := dynamodb.New(sess)
	xray.AWS(dynoClient.Client)

	tableName := "student-db"

	switch request.HTTPMethod {
	case "GET":
		return getStudent(ctx, request, *dynoClient, tableName), nil

	case "PUT":
		return putStudent(ctx, request, *dynoClient, tableName), nil

	case "DELETE":
		return deleteStudent(ctx, request, *dynoClient, tableName), nil

	case "POST":
		return updateStudent(ctx, request, *dynoClient, tableName), nil

	}

	return events.APIGatewayProxyResponse{Body: "Method not allowed", StatusCode: 405}, nil
}

package main

import (
	"context"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-lambda-go/lambda"
	"github.com/aws/aws-sdk-go/aws/session"
	"github.com/aws/aws-sdk-go/service/dynamodb"
)

func main() {
	lambda.Start(lambdaHandler);
}

func lambdaHandler(ctx context.Context, request events.APIGatewayProxyRequest) (events.APIGatewayProxyResponse, error) {
	sess := session.Must(session.NewSessionWithOptions(session.Options{
		SharedConfigState: session.SharedConfigEnable,
	}))

	dynoClient := dynamodb.New(sess);
	
	tableName := "StudentDB"

	switch request.HTTPMethod{
	case "GET":
		return getStudents(request, *dynoClient, tableName), nil
	
	case "PUT":
		return putStudent(request, *dynoClient, tableName), nil
	}

	return events.APIGatewayProxyResponse{Body: "Method not allowed", StatusCode: 405}, nil
}
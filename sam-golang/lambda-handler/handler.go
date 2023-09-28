package main

import (
	"encoding/json"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-sdk-go/service/dynamodb/dynamodbattribute"
)

func getStudents(request events.APIGatewayProxyRequest, dynoClient dynamodb.DynamoDB, tableName string) (events.APIGatewayProxyResponse) {
	rollno := request.QueryStringParameters["rollno"]

	result, err := dynoClient.GetItem(&dynamodb.GetItemInput{
		TableName: aws.String(tableName),
		Key: map[string]*dynamodb.AttributeValue{
			"Rollno": {
				N: aws.String(rollno),
			},
		},
	})

	if err != nil {
		return events.APIGatewayProxyResponse{
			Body:       "Error while Fetching Record: " + err.Error(),
			StatusCode: 500,
		}
	}

	curr_student := Student{}
	dynamodbattribute.UnmarshalMap(result.Item, &curr_student)

	resp_body, _ := json.Marshal(curr_student)

	return events.APIGatewayProxyResponse{Body: string(resp_body), StatusCode: 200}
}

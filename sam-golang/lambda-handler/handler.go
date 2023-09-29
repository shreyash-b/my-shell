package main

import (
	"encoding/json"
	"strconv"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-sdk-go/service/dynamodb/dynamodbattribute"
)

type Student struct {
	Rollno int    `json:"Rollno"`
	Name   string `json:"Name"`
}

func getStudent(request events.APIGatewayProxyRequest, dynoClient dynamodb.DynamoDB, tableName string) events.APIGatewayProxyResponse {
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

func putStudent(request events.APIGatewayProxyRequest, dynoClient dynamodb.DynamoDB, tableName string) events.APIGatewayProxyResponse {
	rollno, err := strconv.Atoi(request.QueryStringParameters["rollno"])
	name := request.QueryStringParameters["name"]

	if err != nil {
		return events.APIGatewayProxyResponse{
			Body:       "Unable to parse rollno: " + err.Error(),
			StatusCode: 400,
		}
	}

	curr_student := Student{
		Rollno: rollno,
		Name:   name,
	}

	item, _ := dynamodbattribute.MarshalMap(curr_student)

	_, err = dynoClient.PutItem(&dynamodb.PutItemInput{
		TableName: &tableName,
		Item:      item,
	})

	if err != nil {
		return events.APIGatewayProxyResponse{
			Body:       "Unable to add value" + err.Error(),
			StatusCode: 500,
		}
	}

	return events.APIGatewayProxyResponse{Body: "Ok", StatusCode: 200}
}

func deleteStudent(request events.APIGatewayProxyRequest, dynoClient dynamodb.DynamoDB, tableName string) events.APIGatewayProxyResponse {
	rollno := request.QueryStringParameters["rollno"]


	_, err := dynoClient.DeleteItem(&dynamodb.DeleteItemInput{
		TableName: &tableName,
		Key: map[string]*dynamodb.AttributeValue{
			"Rollno": {
				N: aws.String(rollno),
			},
		},
	})

	if err!=nil{
		return events.APIGatewayProxyResponse{
			Body:       "Unable to delete: " + err.Error(),
			StatusCode: 500,
		}
	}

	return events.APIGatewayProxyResponse{Body: "Ok", StatusCode: 200}
}

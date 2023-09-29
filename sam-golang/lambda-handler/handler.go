package main

import (
	"encoding/json"

	"github.com/aws/aws-lambda-go/events"
	"github.com/aws/aws-sdk-go/aws"
	"github.com/aws/aws-sdk-go/service/dynamodb"
	"github.com/aws/aws-sdk-go/service/dynamodb/dynamodbattribute"
)

type Student struct {
	Rollno string    `json:"Rollno"`
	Name   string `json:"SName"`
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
	json_data:= Student{}
	// json.Unmarshal(string.NewReader(request.Body))
	json.Unmarshal([]byte(request.Body), &json_data)

	rollno := json_data.Rollno
	student_name := json_data.Name
	
	curr_student := Student{
		Rollno: rollno,
		Name:   student_name,
	}
	
	item, _ := dynamodbattribute.MarshalMap(curr_student)
	
	_, err := dynoClient.PutItem(&dynamodb.PutItemInput{
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
	json_data:= Student{}
	// json.Unmarshal(string.NewReader(request.Body))
	json.Unmarshal([]byte(request.Body), &json_data)

	rollno := json_data.Rollno
	
	
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

func updateStudent(request events.APIGatewayProxyRequest, dynoClient dynamodb.DynamoDB, tableName string) events.APIGatewayProxyResponse  {
	json_data:= Student{}
	// json.Unmarshal(string.NewReader(request.Body))
	json.Unmarshal([]byte(request.Body), &json_data)

	rollno := json_data.Rollno
	student_name := json_data.Name
	
	// in_json := json.NewDecoder(strings.NewReader(request.Body))
	
	
	_, err := dynoClient.UpdateItem(&dynamodb.UpdateItemInput{
		TableName: aws.String(tableName),
		Key: map[string]*dynamodb.AttributeValue{
			"Rollno": {
				N: &rollno,
			},
		},
		ExpressionAttributeValues: map[string]*dynamodb.AttributeValue{
			":r": {
				S: aws.String(student_name),
			},
		},

		UpdateExpression: aws.String("set SName = :r"),
	})
	
	if err!=nil{
		return events.APIGatewayProxyResponse{
			Body:       "Unable to update: " + err.Error(),
			StatusCode: 500,

		}
	}
	
	json_str, _ := json.Marshal(json_data)
	return events.APIGatewayProxyResponse{Body: string(json_str), StatusCode: 200}
}
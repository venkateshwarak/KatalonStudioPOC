#Author: your.email@your.domain.com
#Keywords Summary :
#Feature: List of scenarios.
#Scenario: Business rule through list of steps with arguments.
#Given: Some precondition step
#When: Some key actions
#Then: To observe outcomes or validation
#And,But: To enumerate more Given,When,Then steps
#Scenario Outline: List of steps for data-driven as an Examples and <placeholder>
#Examples: Container for s table
#Background: List of steps run before each of the scenarios
#""" (Doc Strings)
#| (Data Tables)
#@ (Tags/Labels):To group Scenarios
#<> (placeholder)
#""
## (Comments)
#Sample Feature Definition Template
@LoginFeature
Feature: Login
  		Test the login functionality of the application
  I want to use this template for my feature file

  Scenario Outline: Test the login functionality of the MAP's application
    Given user navigates to the login page <url>
    When user enters the username <username> and password <password>
    And user clicks login button
    Then user is navigated to home page
    
    Examples: 
      | username                               | password     | url                                                       |
      | venkateshwara.kayyala@parexelcloud.com | Kvbnithya@84 | https://pxl-awe-dvt-mapweb-ui-teamo-ft.azurewebsites.net/ |

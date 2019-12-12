package com.maps.steps
import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject

import com.kms.katalon.core.annotation.Keyword
import com.kms.katalon.core.checkpoint.Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling
import com.kms.katalon.core.testcase.TestCase
import com.kms.katalon.core.testcase.TestCaseFactory
import com.kms.katalon.core.testdata.TestData
import com.kms.katalon.core.testdata.TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository
import com.kms.katalon.core.testobject.TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI

import internal.GlobalVariable

import org.openqa.selenium.WebElement
import org.openqa.selenium.WebDriver
import org.openqa.selenium.By

import com.kms.katalon.core.mobile.keyword.internal.MobileDriverFactory
import com.kms.katalon.core.webui.driver.DriverFactory

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.testobject.ConditionType
import com.kms.katalon.core.testobject.TestObjectProperty

import com.kms.katalon.core.mobile.helper.MobileElementCommonHelper
import com.kms.katalon.core.util.KeywordUtil

import com.kms.katalon.core.webui.exception.WebElementNotFoundException

import cucumber.api.java.en.And
import cucumber.api.java.en.Given
import cucumber.api.java.en.Then
import cucumber.api.java.en.When



class LoginStep {
	/**
	 * The step definitions below match with Katalon sample Gherkin steps
	 */
	@Given("user navigates to the login page (.*)")
	def NavigateToLoginPage(String url) {
		WebUI.openBrowser('')
		WebUI.navigateToUrl(url)
		WebUI.maximizeWindow()
	}

	//@And('"I enter the user name as "(.*) and password as "(.*)"')
	@When("user enters the username (.*) and password (.*)")
	def EnterUserNameAndPassword(String username,String password){
		WebUI.click(findTestObject('Object Repository/MAPsLogin/Page_GTS-PAA/span_'))
		WebUI.click(findTestObject('Object Repository/MAPsLogin/Page_GTS-PAA/button_Corporate Log In'))
		WebUI.setText(findTestObject('Object Repository/MAPsLogin/Page_Sign in to your account/input_Sign in_loginfmt'), username)
		WebUI.click(findTestObject('Object Repository/MAPsLogin/Page_Sign in to your account/input_Sign in_idSIButton9'))
		//WebUI.wait(2)
		WebUI.setText(findTestObject('Object Repository/MAPsLogin/Page_Sign in to your account/input_Enter password_passwd'),
				password)	}

	@And("user clicks login button")
	def ClickLogin() {	
		WebUI.click(findTestObject('Object Repository/MAPsLogin/Page_Sign in to your account/input_Sign in_idSIButton9'))
		WebUI.click(findTestObject('Object Repository/MAPsLogin/Page_Sign in to your account/input_Do this to reduce the number of times_edee07'))
		WebUI.click(findTestObject('Object Repository/MAPsLogin/Page_Sign in to your account/input_Sign in_idSIButton9'))
		WebUI.click(findTestObject('Object Repository/MAPsLogin/Page_GTS - PAA - Dashboard/button_Got it'))
	}

	@Then("user is navigated to home page")
	def VerifyUserhomepag() {
		//WebUI.verifyElementText(findTestObject('Maps_Login/Page_GTS - PAA - Dashboard/span_Dashboard'), 'Dashboard')
		WebUI.verifyElementText(findTestObject('Object Repository/MAPsLogin/Page_GTS - PAA - Dashboard/span_Dashboard'), 'Dashboard')
	}


}
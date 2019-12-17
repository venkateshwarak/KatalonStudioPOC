<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetPatients</name>
   <tag></tag>
   <elementGuidId>a73f75f2-1500-4c42-b755-d7c833ae066d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>*/*</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiIsImtpZCI6Ilg1ZVhrNHh5b2pORnVtMWtsMll0djhkbE5QNC1jNTdkTzZRR1RWQndhTmsifQ.eyJleHAiOjE1NzYyNDAzNzAsIm5iZiI6MTU3NjIzNjc3MCwidmVyIjoiMS4wIiwiaXNzIjoiaHR0cHM6Ly9sb2dpbi5taWNyb3NvZnRvbmxpbmUuY29tL2Y3MDVlNzQ0LWIzOWYtNDI4Yi1iY2U3LTk3NmMyNGU2ZWYwMi92Mi4wLyIsInN1YiI6ImFmMTZiYjY5LTE0YmMtNGVjOC04NmUzLTE4NDk1NjViODdmMiIsImF1ZCI6IjgzZDFhMmVhLWYwZDUtNDUyZS1hOWYwLTJkNjY5YjgwNzA1ZCIsIm5vbmNlIjoiVkVUcVZ0UElGNWI2RGhISjlOV3JzWUNOWklzd1lyaFoxNTc2MjM2NzUwNjI4IiwiaWF0IjoxNTc2MjM2NzcwLCJhdXRoX3RpbWUiOjE1NzYyMzY3NzAsIm5hbWUiOiJWZW5rYXQgQmFiYSIsImVtYWlscyI6WyJ2ZW5rYXRlc2h3YXJhLmtheXlhbGFAcGFyZXhlbC5jb20iXSwidGZwIjoiQjJDXzFfU2lnbmluU2lnbnVwIn0.OUrfNV_q6CImYenw5KygOcbZYM_KZzrdJu3G1qRAaLnqV6WFqJBf-VVcowIWC5IblBYCUY-pwEi3W5zcNlpViY5o4pe-7cN_6PGurcF0UOhO9AgBsB9Is9SB9vBYVHac5t9M9Dd622h_H6NNAHELpo4ML8Rt9W8e4zaTi779D2-PJjJY1KqtxTlHKO6jF6zXHZm8Nkwvf_KDyOwGZNMaCzwZi5gNiqm3C6XXc2gg1Z7w0D62RO3VeyAAHWiuu42SiTiTLG69D-kbsWULttQSNQYENVNEWlOtyqB6c15StpUHNmdmUW-6kLz88xc3M3cdGT1l79M18FhT0Bml9lMKBQ</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://pxl-awe-dvt-cdpapi-subject-teamo-ft.azurewebsites.net/api/v1/subjects</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable




ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()


RequestObject request = WSResponseManager.getInstance().getCurrentRequest()
WS.verifyElementPropertyValue(response, '[0].PatientGroupId', &quot;ff562668-985d-4e9d-b553-010aeebc7fd6&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

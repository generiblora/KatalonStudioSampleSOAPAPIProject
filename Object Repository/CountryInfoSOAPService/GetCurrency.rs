<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GetCurrency</name>
   <tag></tag>
   <elementGuidId>f1b45a38-3ca7-4728-9362-d65e0d0909af</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
      <webElementGuid>e79df98c-d1cd-4f73-b76f-41e831c75a90</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:web=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;web:CountryCurrency>
         &lt;web:sCountryISOCode>${CountryCode}&lt;/web:sCountryISOCode>
      &lt;/web:CountryCurrency>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso</soapServiceEndpoint>
   <soapServiceFunction>CountryCurrency</soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'IN'</defaultValue>
      <description></description>
      <id>8050dca1-7f06-489f-9da1-741062c2df4b</id>
      <masked>false</masked>
      <name>CountryCode</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

WS.verifyElementText(response, 'CountryCurrencyResponse.CountryCurrencyResult.sName', 'Rupees')</verificationScript>
   <wsdlAddress>http://webservices.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?wsdl</wsdlAddress>
</WebServiceRequestEntity>

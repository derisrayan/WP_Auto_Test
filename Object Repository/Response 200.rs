<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>Response 200 OK</description>
   <name>Response 200</name>
   <tag></tag>
   <elementGuidId>d9499de2-98e4-41a5-b319-e1e93dc31e4f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n   \&quot;prefix\&quot;:\&quot;PT\&quot;,\n   \&quot;name\&quot;:\&quot;Sejahtera\&quot;,\n   \&quot;suffix\&quot;:\&quot;Tbk\&quot;,\n   \&quot;industry_id\&quot;:\&quot;1\&quot;,\n   \&quot;employee_size\&quot;:\&quot;500\&quot;,\n   \&quot;street\&quot;:\&quot;Jl.Sudirman kav. 21\&quot;,\n   \&quot;place\&quot;:\&quot;Sudirman Tower\&quot;,\n   \&quot;geograph_id\&quot;:100,\n   \&quot;phone\&quot;:\&quot;08561290092\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.9.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://run.mocky.io/v3/d81a5cb5-6898-4b9a-8593-855319609168</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

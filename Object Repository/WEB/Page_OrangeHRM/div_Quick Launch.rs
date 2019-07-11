<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>div_Quick Launch</name>
   <tag></tag>
   <elementGuidId>d53fdbcf-4575-4c7e-b3e8-7daffe8e0f4e</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//div[@id='panel_wrapper_0']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>div</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>id</name>
      <type>Main</type>
      <value>panel_wrapper_0</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>class</name>
      <type>Main</type>
      <value>panel_wrapper</value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>text</name>
      <type>Main</type>
      <value>
                                                                                
                                
                                    Quick Launch
                                    

    .quickLinkText{
        display: block;
        text-align: center;
        color: black;
        font-weight:bold;
    }
    a:hover, a:visited, a:link, a:active{
        text-decoration: none;
    }
    div.quickLaunge{
        width: 100px;
        height: 80px;
        vertical-align:middle; 
        text-align:center
    }
    div.quickLaunge img{
        width: 50px;
        height: 50px;
    }
    table.quickLaungeContainer{
        width: auto;
    }



    
        
                                
                                                    
                                
                                    
                                        
                                        Assign Leave
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Leave List
                                    
                                                        
                                                
                                                
                                
                                    
                                        
                                        Timesheets
                                    
                                                        
                                                
                                        
                
        
    



    $(document).ready(function () {
        // hover color change effect
        $(&quot;#dashboard-quick-launch-panel-slider li&quot;).hover(function () {
            $(this).animate({opacity: 0.90}, 100, function () {
                $(this).animate({opacity: 1}, 0);
            });
        });
        // Trigger mouse move event over the 'menu_holder'.
        $(&quot;#dashboard-quick-launch-panel-menu_holder&quot;).mousemove(function (e) {
            // Enable scroll function only when the height of the 'slider' or menu is greater than the 'menu_holder'.
            if ($(this).height() &lt; $(&quot;#dashboard-quick-launch-panel-slider&quot;).height()) {
                // Calculate the distance value from the 'menu_holder' y pos and page Y pos.
                var distance = e.pageY - $(this).offset().top;
                // Get the percentage value with respect to the Mouse Y on the 'menu_holder'.
                var percentage = distance / $(this).height();
                // Calculate the new Y position of the 'slider'. 
                var targetY = -Math.round(($(&quot;#dashboard-quick-launch-panel-slider&quot;).height() - $(this).height()) * percentage);
                // With jQuery easing funtion from easing plugin.
                $('#dashboard-quick-launch-panel-slider').animate({top: [targetY + &quot;px&quot;, &quot;easeOutCirc&quot;]}, {queue: false, duration: 200});
                // Without easeing function. by default jQuery have 'swing'.
                //$('#slider').animate({top: [targetY+&quot;px&quot;, &quot;easeOutCirc&quot;]}, { queue:false, duration:200 });
            }
        });
    });

                                 
                            

                                            </value>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>id(&quot;panel_wrapper_0&quot;)</value>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//div[@id='panel_wrapper_0']</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:idRelative</name>
      <type>Main</type>
      <value>//div[@id='group_0']/div</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Dashboard'])[2]/following::div[4]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:neighbor</name>
      <type>Main</type>
      <value>(.//*[normalize-space(text()) and normalize-space(.)='Access Records'])[1]/following::div[7]</value>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:position</name>
      <type>Main</type>
      <value>//div[2]/div/div/div</value>
   </webElementXpaths>
</WebElementEntity>

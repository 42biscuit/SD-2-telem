# SD-2-telem
This is the plotting software supporting the telemetry system for The University of Sheffield Student lead project SD Squared. (Sheffield Student Downhill Design)


## Project Programming Methodology
- Give all variables concise descriptive names
- For all functions handeling or manipulating data imagin the function is a black box, what are the ouputs you could expect for given inputs. What are errenous inputs, how will the function handel them. Write tests for these before you write the function
- Follow the DRY (Dont Repeat Yourself) Principal
- Write documentation as you go + concise comments. If you came back in 6 months would you be able to find how the function works.
- If you have questions about using the Git drop someone a message.


         Load data 
            |                  
      calculate views  ---------- data is stored in telemData    
            |
     store in sus_views
            |
display views to match plot

sus_views is
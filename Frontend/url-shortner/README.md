1. Create a basic view layout
    a. Have a basic Header
    b. Have an input for entering the Url and a button to create and add the short url
    c. Have a Table with full url, short url and clicks
    d. Make sure the Table is responsive
2. On Shorten Button click
    a. Add this to a Hash-Table with fullUrl (key), shortened-url (uniq id) and clicks (values)
    b. If the entered url is already been shortened,
        i. Re-use the existing shortened-url
    c. Otherwise, 
        i. add the entered url to fullUrl
        ii. Generate a uniq id and add it as shortened-url
        iii. Initialize clicks to 0
    d. Create a small box next to the input box with the link pointing to the fullUrl but       displaying the shortened-url.
    e. If clicked, increment the click by one.
    f. Update the Table Display
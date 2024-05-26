Regarding customization, I really don't know how I should approach the problem.

***Restructure with App Structs that contain ID, display name, exec, and AppInfo ***
Do this so it becomes easier to look up during the search w/ regex
Hash doesn't store enough info that we would need.

*********
We need to look at two possible options:
1. insert all list box rows at beginning of launch,
then sort and re order then as you search
2. Insert all list box rows at beginning of launch, then remove all and only return 10 at a time,
and clear them every time you do a search change.

We should store the app infos, in a Vector. Then we can modify the vector, and make sure it doesn't have any duplicate AppInfos. 
Then, we iterate through the vector, and add a row for each item 
that is NOT duplicated.

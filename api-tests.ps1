Invoke-RestMethod -Uri http://localhost:6570/

Invoke-RestMethod -Uri http://localhost:6570/vehicle Method Get

Invoke-RestMethod -Uri http://localhost:6570/vehicle Method Post

$Params = @{
    Uri = 'http://localhost:6570/vehicle'
    Method = 'Post'
    Body = @{
        manufacturer = 'Tesla'
        model = 'Model Y'
        year = 2024
    }|ConvertTo-Json
    ContentType ='application/json'
}
Invoke-RestMethod @Params
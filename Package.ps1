cargo build --release
Remove-Item packaged -Force
New-Item -ItemType Directory packaged
Copy-Item target\release\app.exe packaged
Copy-Item target\release\app.pdb packaged
Copy-Item packaging\* packaged

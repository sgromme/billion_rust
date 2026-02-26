 # looking at data first 10 rows
 head -n 10 /home/sgromme/source/1brc/data/measurements.txt

# time loading
time cat /home/sgromme/source/1brc/data/measurements.txt > /dev/null

sgromme@DESKTOP-397HU7D:~/source/billion_rust$ time cat /home/sgromme/source/1brc/data/measurements.txt > /dev/null

real    4m45.223s
user    0m0.426s
sys     4m26.589s


# create a small subset of data in current directory
head -n 10 /home/sgromme/source/1brc/data/measurements.txt > measurements.txt




#  How to parallize the data

Memory map file? partition it, hash values , then merge and 
what about 






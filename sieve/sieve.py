import sys
import math

def sieve(max):
	#Takes in a number, and returns all primes between 2 and that number
	
	#Start with all of the numbers
	primes = list(range(2,max+1))
	#Start running through each number 
	i = 0
	done = False
	for i in range(0, int(math.sqrt(max))):
		if primes[i] == 0:
			continue
		#Start with double the number, and
		j = i + primes[i] 
		#remove all multiples
		while j < len(primes):
			primes[j] = 0
			j += primes[i]
		

	
	return list(filter( lambda x: x != 0, primes) )

for prime in sieve(int(sys.argv[1])):
	print(prime)

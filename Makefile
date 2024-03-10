python_src = plot_spectrum
gen_rule = ex_symtop_spectrum_2
res = result_${gen_rule}.csv

plot: ${res} 
	poetry run python ${python_src}.py $<

${res}:
	cargo run --example ${gen_rule} ${res} 

clean:
	rm *.log *.csv

.PHONY: plot clean

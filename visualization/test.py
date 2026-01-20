import root_finder
import itertools
import matplotlib.pyplot as plt

def generate_and_plot_roots(max_degree, cf_range):
    real_parts = []
    im_parts = []

    possible_coeffs = list(range(-cf_range, cf_range + 1))

    for cfs in itertools.product(possible_coeffs, repeat = max_degree + 1):
    
        if cfs[-1] == 0:
            continue

        roots = root_finder.companion(list(cfs))

        for real, imag in roots:
            real_parts.append(real)
            im_parts.append(imag)

    plt.figure(figsize=(10,10))

    plt.scatter(real_parts, im_parts, s = 0.5, alpha = 0.5, color = 'blue')

    plt.axhline(0, color='black', lw=1)
    plt.axvline(0, color='black', lw=1)
    plt.grid(True, linestyle='--', alpha=0.6)
    plt.show()






generate_and_plot_roots(max_degree=4, cf_range=3)

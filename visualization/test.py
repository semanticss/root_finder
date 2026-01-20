import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
import time
import root_finder

def run_simulation(max_degree, cf_range):
    print(f"--- Starting Simulation: Degree {max_degree}, Range {cf_range} ---")
    
    start_setup = time.perf_counter()
    vals = np.arange(-cf_range, cf_range + 1, dtype=np.float64)
    
    # Generate the coordinate grids
    grid_tuple = np.meshgrid(*[vals] * (max_degree + 1), indexing='ij')
    
    # Stack them along a new axis and reshape to get (N, degree + 1)
    grid = np.stack(grid_tuple, axis=-1).reshape(-1, max_degree + 1)
    
    # Filter: Leading coefficient (last column) cannot be 0
    grid = grid[grid[:, -1] != 0]
    
    count = grid.shape[0]
    flat_coeffs = grid.ravel().tolist()
    
    print(f"Setup time: {time.perf_counter() - start_setup:.4f}s | Polynomials: {count:,}")

    start_rust = time.perf_counter()
    # Ensure Rust takes: (flat_coeffs: Vec<f64>, degree: usize)
    re, im = root_finder.batching_companion(flat_coeffs, max_degree)
    print(f"Rust math time: {time.perf_counter() - start_rust:.4f}s")

    print("Rendering heatmap...")
    plt.figure(figsize=(12, 12), facecolor='black')
    
    heatmap, xedges, yedges = np.histogram2d(
        re, im, bins=1200, range=[[-2, 2], [-2, 2]]
    )

    plt.imshow(
        heatmap.T, 
        extent=[xedges[0], xedges[-1], yedges[0], yedges[-1]], 
        origin='lower', 
        cmap='magma'
    )
    
    plt.axis('off')
    plt.savefig("fractal_roots.png", dpi=300, bbox_inches='tight', facecolor='black')
    print("Saved to fractal_roots.png")

if __name__ == "__main__":
    # This should run very fast now
    run_simulation(max_degree=6, cf_range=2)
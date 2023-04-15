import matplotlib.pyplot as plt

class PointProcessPlotting:
    def __init__(self, inhomogeneous_events, T, num_bins):
        self.inhomogeneous_events = inhomogeneous_events
        self.T = T
        self.num_bins = num_bins
        
        self.compute_hist(inhomogeneous_events, T, num_bins, bin_duration, num_bins)
        self.plot()
        
    # Compute hist
    def compute_hist(self, events, T, bins, bin_duration , num_bins):
        
        # Calculate the bin edges
        self.bin_edges = np.arange(0, T, bin_duration)[:num_bins+1]
        self.hist, _ = np.histogram(events, bins=bins, range=(0, T))
            
    def plot(self):
        
        # Create a 1x2 grid of subplots
        fig, axes = plt.subplots(nrows=2, ncols=1, figsize=(12, 4), sharex=True)

        # Plot the event plot
        axes[0].eventplot(self.inhomogeneous_events, linelengths=0.8, color='black')
        axes[0].set_xlabel('Time')
        axes[0].set_ylabel('Events')
        axes[0].set_title('Event plot with Gaussian intensity')
        axes[0].grid()
        axes[0].set_xlim(0, T)

        # Plot the histogram
        axes[1].bar(self.bin_edges, self.hist, width = bin_duration, align='edge', edgecolor='black')
        axes[1].set_xlabel('Time')
        axes[1].set_ylabel('Event Count')
        axes[1].set_title('Event histogram with Gaussian intensity')
        axes[1].grid()

        # Display the plots
        plt.tight_layout()
        plt.show()

import os
import numpy as np 

print(os.getcwd())

T = 10 
peak_time_of_kernel = 5 
width_of_kernel = 1
peak_intensity_of_kernel = 50
num_bins = T
bin_duration = T / num_bins
lam = 10

file =  os.getcwd() + "/spike_times.csv"
data = np.loadtxt(file)

plot_object = PointProcessPlotting(inhomogeneous_events = data,
                                       T = T,
                                       num_bins = num_bins)


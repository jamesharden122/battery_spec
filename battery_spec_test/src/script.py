class SolarPVSystem:
    def __init__(self, latitude, longitude, module_parameters=None, inverter_name='ABB__MICRO_0_25_I_OUTD_US_208__208V_',surface_tilt=20, 
        surface_azimuth=180, panels_per_array=10, number_of_arrays=1, watts = 450):
            import pvlib
            from pvlib import location, modelchain
            from pvlib.temperature import TEMPERATURE_MODEL_PARAMETERS
            ##Location: 
            self.latitude = latitude
            self.longitude = longitude
            self.location = location.Location(latitude, longitude)
            self.temperature_model_parameters = TEMPERATURE_MODEL_PARAMETERS['sapm']['open_rack_glass_glass']
            ##System Design
            # Initialize arrays
            # Retrieve inverter parameters from the SAM database
            sandia_modules = pvlib.pvsystem.retrieve_sam('SandiaMod')
            sandia_mod = sandia_modules['Canadian_Solar_CS5P_220M___2009_']
            # Retrieve the Sandia Module database
            # Now update the module with new specifications for a 450W panel
            sandia_mod['A0'] = 0.97
            sandia_mod['A1'] = 0.065
            sandia_mod['A2'] = -0.015
            sandia_mod['A3'] = 0.0017
            sandia_mod['A4'] = -0.00007
            sandia_mod['B0'] = 1
            sandia_mod['B1'] = -0.0025
            sandia_mod['B2'] = 0.0004
            sandia_mod['B3'] = -0.00002
            sandia_mod['B4'] = 0.0
            sandia_mod['B5'] = -0.0
            sandia_mod['DTC'] = 3
            sandia_mod['FD'] = 1
            sandia_mod['A'] = -3.56
            sandia_mod['B'] = -0.075
            sandia_mod['C4'] = 1.0
            sandia_mod['C5'] = 0.0
            sandia_mod['IXO'] = 10.22
            sandia_mod['IXXO'] = 6.30
            sandia_mod['C6'] = 1.1
            sandia_mod['C7'] = -0.1
            # Update the electrical parameters
            sandia_mod['Isco'] = 10.22
            sandia_mod['Voco'] = 48.76
            sandia_mod['Impo'] = 9.61
            sandia_mod['Vmpo'] = 41.6
            sandia_mod['N'] = 1.5
            sandia_mod['Cells_in_Series'] = 144
            sandia_mod['Parallel_Strings'] = 1
            sandia_mod['Area'] = 2.0
            # Temperature coefficients
            sandia_mod['Bvoco'] = -0.123
            sandia_mod['Bvmpo'] = -0.098
            inverters = pvlib.pvsystem.retrieve_sam('cecinverter')
            self.inverter_parameters = inverters[inverter_name]
            # Update inverter parameters
            # NOTE: These values are hypothetical; use actual data for your specific inverter and panels
            self.inverter_parameters['Pdc0'] = 400 * 1.1  # Max DC power capacity, 10% above system size
            self.inverter_parameters['Vdcmax'] = 600  # Max DC voltage, adjust based on your panel configuration
            self.inverter_parameters['Mppt_low'] = 200  # Lower bound of MPPT range, adjust based on panel specs
            self.inverter_parameters['Mppt_high'] = 500  # Upper bound of MPPT range, adjust based on panel specs

            
            self.arrays = []
            for _ in range(number_of_arrays):
                mount = pvlib.pvsystem.FixedMount(surface_tilt=surface_tilt, surface_azimuth=surface_azimuth)
                array = pvlib.pvsystem.Array(mount=mount, 
                                            module_parameters= sandia_mod, 
                                            temperature_model_parameters = self.temperature_model_parameters, 
                                            modules_per_string =1)
                self.arrays.append(array)
                print(self.arrays)
            
            # Instantiate PVSystem with multiple arrays
            
            self.system = pvlib.pvsystem.PVSystem(arrays=self.arrays, 
                                                  inverter_parameters=self.inverter_parameters)
            # Initialize ModelChain with this system and
            self.mc = modelchain.ModelChain(self.system, self.location, aoi_model="no_loss", spectral_model="no_loss")

    def calculate_power_output(self, date1, date2, epw):
        import pandas as pd
        from datetime import datetime
        # Run the ModelChain for the given weather data
        weather_data = pd.DataFrame({
            'year': epw["year"].to_list(),'month': epw["month"].to_list(),
            'day': epw["day"].to_list(),'hour': epw["hour"].to_list(),
            'ghi': epw["ghi"].to_list(), 'dni': epw["dni"].to_list(), 'dhi': epw["dhi"].to_list(), 
            'temp_air': epw["temp_air"].to_list(), 'wind_speed': epw["wind_speed"].to_list()
        }, index=pd.to_datetime(epw["date"],utc = True))
        print("got here")
        self.mc.run_model(weather_data)
        print("done")
        # Access and return the calculated DC and AC power
        return self.mc.results.dc, self.mc.results.ac 

import polars
import pandas as pd 
if __name__ == "__main__":
    # Simulate weather data for demonstration
    # Instantiate the SolarPVSystem

    solar_system = SolarPVSystem(latitude, longitude, panels_per_array=panels_per_array, 
                                number_of_arrays=number_of_arrays, watts = panel_watts)
    # Calculate power output for the specified weather data
    # Convert Polars DataFrame to JSON
    print("got here")
    # Convert JSON string to Pandas DataFrame
    weather_data = weather_data.select(['date','year','month','day','hour','ghi', 'dni', 'dhi', 'temp_air', 'wind_speed'])
    dc_power, ac_power = solar_system.calculate_power_output(date1, date2, weather_data)
    dc_power[0].to_csv('dc_power.csv', index=True)
    ac_power.to_csv('ac_power.csv', index=True)
    print(dc_power[0].head(40))
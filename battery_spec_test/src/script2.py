import pvlib
from pvlib import location, modelchain
from pvlib.temperature import TEMPERATURE_MODEL_PARAMETERS
from pvlib.irradiance import get_total_irradiance
from pvlib.pvarray import pvefficiency_adr


df = pd.DataFrame({'ghi': tmy['ghi'], 'dhi': tmy['dhi'], 'dni': tmy['dni'],
                   'temp_air': tmy['temp_air'],
                   'wind_speed': tmy['wind_speed'],
                   })

PVLIB_DIR = pvlib.__path__[0]
DATA_FILE = os.path.join(PVLIB_DIR, 'data', '723170TYA.CSV')

tmy, metadata = iotools.read_tmy3(DATA_FILE, coerce_year=1990,
                                  map_variables=True)

df.index = df.index - pd.Timedelta(minutes=30)

loc = location.Location.from_tmy(metadata)
solpos = loc.get_solarposition(df.index)

TILT = metadata['latitude']
ORIENT = 180

total_irrad = get_total_irradiance(TILT, ORIENT,
                                   solpos.apparent_zenith, solpos.azimuth,
                                   df.dni, df.ghi, df.dhi)

df['poa_global'] = total_irrad.poa_global
df['temp_pv'] = pvlib.temperature.faiman(df.poa_global, df.temp_air,
                                         df.wind_speed)
# Borrow the ADR model parameters from the other example:

adr_params = {'k_a': 0.99924,
              'k_d': -5.49097,
              'tc_d': 0.01918,
              'k_rs': 0.06999,
              'k_rsh': 0.26144
              }

df['eta_rel'] = pvefficiency_adr(df['poa_global'], df['temp_pv'], **adr_params)

# Set the desired array size:
P_STC = 5000.   # (W)

# and the irradiance level needed to achieve this output:
G_STC = 1000.   # (W/m2)

df['p_mp'] = P_STC * df['eta_rel'] * (df['poa_global'] / G_STC)